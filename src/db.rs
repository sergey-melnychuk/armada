use std::{
    fs::{self, File},
    io::{Read, Write},
    marker::PhantomData,
    path::{Path, PathBuf},
};

use serde::{de::DeserializeOwned, Serialize};

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
    fn get(&self, key: &str) -> anyhow::Result<Option<T>>;
    fn del(&self, key: &str) -> anyhow::Result<Option<T>>;
    fn put(&self, key: &str, val: T) -> anyhow::Result<()>;
}

#[derive(Clone)]
pub struct DirRepo<T: Serialize + DeserializeOwned> {
    base: PathBuf,
    _phantom: PhantomData<T>,
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

    fn get(&self, key: &str) -> anyhow::Result<Option<T>> {
        let mut path = self.base.clone();
        path.push(format!("{}.json", key));
        if !path.exists() {
            return Ok(None);
        }

        let mut file = File::open(&path)?;

        let mut json = String::with_capacity(1024);
        let _ = file.read_to_string(&mut json)?;

        let val: T = serde_json::from_str(&json)?;
        Ok(Some(val))
    }

    fn del(&self, key: &str) -> anyhow::Result<Option<T>> {
        let mut path = self.base.clone();
        path.push(format!("{}.json", key));
        if !path.exists() {
            return Ok(None);
        }

        let mut file = File::open(&path)?;

        let mut json = String::with_capacity(1024);
        let _ = file.read_to_string(&mut json)?;

        let val: T = serde_json::from_str(&json)?;
        fs::remove_file(path)?;
        Ok(Some(val))
    }

    fn put(&self, key: &str, val: T) -> anyhow::Result<()> {
        let mut path = self.base.clone();
        path.push(format!("{}.json", key));
        let mut file = File::create(&path)?;

        let json = serde_json::to_string(&val)?;
        file.write_all(json.as_bytes())?;

        Ok(())
    }
}
