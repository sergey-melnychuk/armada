use std::{
    fs::{self, File},
    io::{Read, Write},
    marker::PhantomData,
    path::{Path, PathBuf},
};

use serde::{de::DeserializeOwned, Serialize};
use yakvdb::{api::tree::Tree, disk::block::Block, disk::file::File as YakFile};

use crate::api::gen::BlockWithTxs;

#[derive(Clone)]
pub struct Storage {
    blocks: DirRepo<BlockWithTxs>,
}

impl Storage {
    pub fn new<P: AsRef<Path>>(base: P) -> Self {
        fs::create_dir_all(base.as_ref()).ok();

        let base = base.as_ref();

        let mut blocks = base.to_owned();
        blocks.push("block");
        let blocks = DirRepo::new(&blocks);

        Self { blocks }
    }

    pub fn blocks(&self) -> &DirRepo<BlockWithTxs> {
        &self.blocks
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

pub struct FileIndex<K, V> {
    file: YakFile<Block>,
    _phantom: PhantomData<(K, V)>,
}

impl<K, V> Index<K, V> for FileIndex<K, V>
where
    K: AsRef<[u8]> + for<'a> From<&'a [u8]>,
    V: AsRef<[u8]> + for<'a> From<&'a [u8]>,
{
    fn new(path: &Path) -> Self {
        let file: YakFile<Block> = if !path.exists() {
            YakFile::make(path, 4096).unwrap()
        } else {
            YakFile::open(path).unwrap()
        };
        Self {
            file,
            _phantom: PhantomData,
        }
    }

    fn has(&self, key: &K) -> anyhow::Result<bool> {
        Ok(self.get(key)?.is_some())
    }

    fn get(&self, key: &K) -> anyhow::Result<Option<V>> {
        Ok(self
            .file
            .lookup(key.as_ref())?
            .as_ref()
            .map(|bytes| V::from(bytes)))
    }

    fn del(&mut self, key: &K) -> anyhow::Result<Option<V>> {
        let val = self.get(key)?;
        self.file.remove(key.as_ref())?;
        Ok(val)
    }

    fn put(&mut self, key: &K, val: V) -> anyhow::Result<()> {
        Ok(self.file.insert(key.as_ref(), val.as_ref())?)
    }

    fn min(&self) -> anyhow::Result<Option<K>> {
        Ok(self.file.min()?.as_ref().map(|bytes| K::from(bytes)))
    }

    fn max(&self) -> anyhow::Result<Option<K>> {
        Ok(self.file.max()?.as_ref().map(|bytes| K::from(bytes)))
    }
}
