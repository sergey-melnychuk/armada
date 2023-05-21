use std::{
    fs::{self, File},
    io::{Read, Write},
    marker::PhantomData,
    path::{Path, PathBuf},
    sync::Arc,
};

use serde::{de::DeserializeOwned, Serialize};
use tokio::sync::RwLock;
use yakvdb::typed::{Store, DB};

use crate::{
    api::gen::BlockWithTxs,
    util::{U256, U64},
};

#[derive(Clone)]
pub struct Storage {
    pub blocks: DirRepo<BlockWithTxs>,
    pub blocks_index: Arc<RwLock<Store<U64, U256>>>,
}

impl Storage {
    pub fn new<P: AsRef<Path>>(base: P) -> Self {
        fs::create_dir_all(base.as_ref()).ok();

        let base = base.as_ref();

        let mut path = base.to_owned();
        path.push("block");
        let blocks = DirRepo::new(&path);

        let mut path = base.to_owned();
        path.push("block");
        path.push("block_number_to_block_hash.yak");
        let blocks_index = Store::new(&path);
        let blocks_index = Arc::new(RwLock::new(blocks_index));

        Self {
            blocks,
            blocks_index,
        }
    }
}

pub trait Repo<T: Serialize + DeserializeOwned> {
    fn new(base: &Path) -> Self;
    fn has(&self, key: &str) -> anyhow::Result<bool>;
    fn get(&self, key: &str) -> anyhow::Result<Option<T>>;
    fn del(&self, key: &str) -> anyhow::Result<Option<T>>;
    fn put(&self, key: &str, val: T) -> anyhow::Result<()>;
}

#[derive(Clone)]
pub struct DirRepo<T: Serialize + DeserializeOwned> {
    base: PathBuf,
    _phantom: PhantomData<T>,
}

impl<T> DirRepo<T>
where
    T: Serialize + DeserializeOwned,
{
    fn path(&self, key: &str) -> PathBuf {
        let mut path = self.base.clone();
        path.push(format!("{}.json", key));
        path
    }

    fn file(&self, key: &str) -> anyhow::Result<Option<String>> {
        let path = self.path(key);
        if !path.exists() {
            return Ok(None);
        }

        let mut file = File::open(&path)?;
        let mut json = String::with_capacity(1024);
        let _ = file.read_to_string(&mut json)?;
        Ok(Some(json))
    }
}

impl<T> Repo<T> for DirRepo<T>
where
    T: Serialize + DeserializeOwned,
{
    fn new(base: &Path) -> Self {
        fs::create_dir_all(base).ok();

        Self {
            base: base.to_owned(),
            _phantom: PhantomData,
        }
    }

    fn has(&self, key: &str) -> anyhow::Result<bool> {
        Ok(self.path(key).exists())
    }

    fn get(&self, key: &str) -> anyhow::Result<Option<T>> {
        let json = self.file(key)?;
        if let Some(json) = json {
            let val: T = serde_json::from_str(&json)?;
            return Ok(Some(val));
        }
        Ok(None)
    }

    fn del(&self, key: &str) -> anyhow::Result<Option<T>> {
        let opt = self.get(key)?;
        if opt.is_some() {
            fs::remove_file(self.path(key))?;
        }
        Ok(opt)
    }

    fn put(&self, key: &str, val: T) -> anyhow::Result<()> {
        let path = self.path(key);
        let mut file = File::create(path)?;
        let json = serde_json::to_string(&val)?;
        file.write_all(json.as_bytes())?;
        Ok(())
    }
}

pub trait Index<K, V>
where
    K: AsRef<[u8]> + for<'a> From<&'a [u8]>,
    V: AsRef<[u8]> + for<'a> From<&'a [u8]>,
{
    fn new(path: &Path) -> Self;
    fn has(&self, key: &K) -> anyhow::Result<bool>;
    fn get(&self, key: &K) -> anyhow::Result<Option<V>>;
    fn del(&mut self, key: &K) -> anyhow::Result<Option<V>>;
    fn put(&mut self, key: &K, val: V) -> anyhow::Result<()>;

    fn min(&self) -> anyhow::Result<Option<K>>;
    fn max(&self) -> anyhow::Result<Option<K>>;
    // TODO: add iter(range), above, below?
}
