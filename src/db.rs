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
    seq::dto,
    util::{U256, U64},
};

#[derive(Clone)]
pub struct Storage {
    pub blocks: DirRepo<BlockWithTxs>,
    pub blocks_index: Arc<RwLock<Store<U64, U256>>>,
    pub txs_index: Arc<RwLock<Store<U256, BlockAndIndex>>>,
    pub states: DirRepo<dto::StateUpdate>,
}

pub struct BlockAndIndex([u8; 40]);

impl BlockAndIndex {
    pub fn from(block: U256, index: U64) -> Self {
        let mut bytes = [0u8; 40];
        bytes[0..32].copy_from_slice(block.as_ref());
        bytes[32..].copy_from_slice(index.as_ref());
        Self(bytes)
    }
    pub fn block(&self) -> U256 {
        U256::from(&self.0[0..32])
    }
    pub fn index(&self) -> U64 {
        U64::from(&self.0[32..])
    }
}

impl AsRef<[u8]> for BlockAndIndex {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl<'a> From<&'a [u8]> for BlockAndIndex {
    fn from(value: &'a [u8]) -> Self {
        let mut bytes = [0u8; 40];
        bytes.copy_from_slice(value);
        Self(bytes)
    }
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
        path.push("index.yak");
        let blocks_index = Store::new(&path);
        let blocks_index = Arc::new(RwLock::new(blocks_index));

        let mut path = base.to_owned();
        path.push("tx");
        fs::create_dir_all(&path).ok();

        let mut path = base.to_owned();
        path.push("tx");
        path.push("index.yak");
        let txs_index = Store::new(&path);
        let txs_index = Arc::new(RwLock::new(txs_index));

        let mut path = base.to_owned();
        path.push("state");
        let states = DirRepo::new(&path);

        Self {
            blocks,
            blocks_index,
            txs_index,
            states,
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
    // TODO: add range loookup (returning iterator)
}
