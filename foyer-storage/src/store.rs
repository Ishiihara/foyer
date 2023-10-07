//  Copyright 2023 MrCroxx
//
//  Licensed under the Apache License, Version 2.0 (the "License");
//  you may not use this file except in compliance with the License.
//  You may obtain a copy of the License at
//
//  http://www.apache.org/licenses/LICENSE-2.0
//
//  Unless required by applicable law or agreed to in writing, software
//  distributed under the License is distributed on an "AS IS" BASIS,
//  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//  See the License for the specific language governing permissions and
//  limitations under the License.

use std::sync::Arc;

use foyer_common::code::{Key, Value};
use foyer_intrusive::eviction::{
    fifo::{Fifo, FifoLink},
    lfu::{Lfu, LfuLink},
    lru::{Lru, LruLink},
};

use crate::{
    device::fs::FsDevice,
    error::Result,
    generic::{GenericStore, GenericStoreConfig, GenericStoreWriter},
    region_manager::RegionEpItemAdapter,
    storage::{ForceStorageWriter, Storage, StorageWriter},
};

pub type LruFsStore<K, V> =
    GenericStore<K, V, FsDevice, Lru<RegionEpItemAdapter<LruLink>>, LruLink>;

pub type LruFsStoreConfig<K, V> =
    GenericStoreConfig<K, V, FsDevice, Lru<RegionEpItemAdapter<LruLink>>>;

pub type LruFsStoreWriter<'w, K, V> =
    GenericStoreWriter<'w, K, V, FsDevice, Lru<RegionEpItemAdapter<LruLink>>, LruLink>;

pub type LfuFsStore<K, V> =
    GenericStore<K, V, FsDevice, Lfu<RegionEpItemAdapter<LfuLink>>, LfuLink>;

pub type LfuFsStoreConfig<K, V> =
    GenericStoreConfig<K, V, FsDevice, Lfu<RegionEpItemAdapter<LfuLink>>>;

pub type LfuFsStoreWriter<'w, K, V> =
    GenericStoreWriter<'w, K, V, FsDevice, Lfu<RegionEpItemAdapter<LfuLink>>, LfuLink>;

pub type FifoFsStore<K, V> =
    GenericStore<K, V, FsDevice, Fifo<RegionEpItemAdapter<FifoLink>>, FifoLink>;

pub type FifoFsStoreConfig<K, V> =
    GenericStoreConfig<K, V, FsDevice, Fifo<RegionEpItemAdapter<FifoLink>>>;

pub type FifoFsStoreWriter<'w, K, V> =
    GenericStoreWriter<'w, K, V, FsDevice, Fifo<RegionEpItemAdapter<FifoLink>>, FifoLink>;

#[derive(Debug)]
pub enum StoreConfig<K, V>
where
    K: Key,
    V: Value,
{
    LruFsStoreConfig { config: LruFsStoreConfig<K, V> },
    LfuFsStoreConfig { config: LfuFsStoreConfig<K, V> },
    FifoFsStoreConfig { config: FifoFsStoreConfig<K, V> },
    None,
}

impl<K, V> From<LruFsStoreConfig<K, V>> for StoreConfig<K, V>
where
    K: Key,
    V: Value,
{
    fn from(config: LruFsStoreConfig<K, V>) -> Self {
        StoreConfig::LruFsStoreConfig { config }
    }
}

impl<K, V> From<LfuFsStoreConfig<K, V>> for StoreConfig<K, V>
where
    K: Key,
    V: Value,
{
    fn from(config: LfuFsStoreConfig<K, V>) -> Self {
        StoreConfig::LfuFsStoreConfig { config }
    }
}

impl<K, V> From<FifoFsStoreConfig<K, V>> for StoreConfig<K, V>
where
    K: Key,
    V: Value,
{
    fn from(config: FifoFsStoreConfig<K, V>) -> Self {
        StoreConfig::FifoFsStoreConfig { config }
    }
}

#[derive(Debug)]
pub enum StoreWriter<'a, K, V>
where
    K: Key,
    V: Value,
{
    LruFsStorWriter { writer: LruFsStoreWriter<'a, K, V> },
    LfuFsStorWriter { writer: LfuFsStoreWriter<'a, K, V> },
    FifoFsStoreWriter { writer: FifoFsStoreWriter<'a, K, V> },
    None,
}

impl<'a, K, V> From<LruFsStoreWriter<'a, K, V>> for StoreWriter<'a, K, V>
where
    K: Key,
    V: Value,
{
    fn from(writer: LruFsStoreWriter<'a, K, V>) -> Self {
        StoreWriter::LruFsStorWriter { writer }
    }
}

impl<'a, K, V> From<LfuFsStoreWriter<'a, K, V>> for StoreWriter<'a, K, V>
where
    K: Key,
    V: Value,
{
    fn from(writer: LfuFsStoreWriter<'a, K, V>) -> Self {
        StoreWriter::LfuFsStorWriter { writer }
    }
}

impl<'a, K, V> From<FifoFsStoreWriter<'a, K, V>> for StoreWriter<'a, K, V>
where
    K: Key,
    V: Value,
{
    fn from(writer: FifoFsStoreWriter<'a, K, V>) -> Self {
        StoreWriter::FifoFsStoreWriter { writer }
    }
}

#[derive(Debug)]
pub enum Store<K, V>
where
    K: Key,
    V: Value,
{
    LruFsStore { store: Arc<LruFsStore<K, V>> },
    LfuFsStore { store: Arc<LfuFsStore<K, V>> },
    FifoFsStore { store: Arc<FifoFsStore<K, V>> },
    None,
}

impl<K, V> Clone for Store<K, V>
where
    K: Key,
    V: Value,
{
    fn clone(&self) -> Self {
        match self {
            Self::LruFsStore { store } => Self::LruFsStore {
                store: Arc::clone(store),
            },
            Self::LfuFsStore { store } => Self::LfuFsStore {
                store: Arc::clone(store),
            },
            Self::FifoFsStore { store } => Self::FifoFsStore {
                store: Arc::clone(store),
            },
            Self::None => Self::None,
        }
    }
}

impl<'a, K, V> StorageWriter for StoreWriter<'a, K, V>
where
    K: Key,
    V: Value,
{
    type Key = K;
    type Value = V;

    fn judge(&mut self) -> bool {
        match self {
            StoreWriter::LruFsStorWriter { writer } => writer.judge(),
            StoreWriter::LfuFsStorWriter { writer } => writer.judge(),
            StoreWriter::FifoFsStoreWriter { writer } => writer.judge(),
            StoreWriter::None => false,
        }
    }

    async fn finish(self, value: Self::Value) -> Result<bool> {
        match self {
            StoreWriter::LruFsStorWriter { writer } => writer.finish(value).await,
            StoreWriter::LfuFsStorWriter { writer } => writer.finish(value).await,
            StoreWriter::FifoFsStoreWriter { writer } => writer.finish(value).await,
            StoreWriter::None => Ok(false),
        }
    }
}

impl<'a, K, V> ForceStorageWriter for StoreWriter<'a, K, V>
where
    K: Key,
    V: Value,
{
    fn set_force(&mut self) {
        match self {
            StoreWriter::LruFsStorWriter { writer } => writer.set_force(),
            StoreWriter::LfuFsStorWriter { writer } => writer.set_force(),
            StoreWriter::FifoFsStoreWriter { writer } => writer.set_force(),
            StoreWriter::None => {}
        }
    }
}

impl<K, V> Storage for Store<K, V>
where
    K: Key,
    V: Value,
{
    type Key = K;
    type Value = V;
    type Config = StoreConfig<K, V>;
    type Owned = Self;
    type Writer<'a> = StoreWriter<'a, K, V>;

    async fn open(config: Self::Config) -> Result<Self::Owned> {
        match config {
            StoreConfig::LruFsStoreConfig { config } => {
                let store = LruFsStore::open(config).await?;
                Ok(Self::LruFsStore { store })
            }
            StoreConfig::LfuFsStoreConfig { config } => {
                let store = LfuFsStore::open(config).await?;
                Ok(Self::LfuFsStore { store })
            }
            StoreConfig::FifoFsStoreConfig { config } => {
                let store = FifoFsStore::open(config).await?;
                Ok(Self::FifoFsStore { store })
            }
            StoreConfig::None => Ok(Self::None),
        }
    }

    async fn close(&self) -> Result<()> {
        match self {
            Store::LruFsStore { store } => store.close().await,
            Store::LfuFsStore { store } => store.close().await,
            Store::FifoFsStore { store } => store.close().await,
            Store::None => Ok(()),
        }
    }

    fn writer(&self, key: Self::Key, weight: usize) -> Self::Writer<'_> {
        match self {
            Store::LruFsStore { store } => store.writer(key, weight).into(),
            Store::LfuFsStore { store } => store.writer(key, weight).into(),
            Store::FifoFsStore { store } => store.writer(key, weight).into(),
            Store::None => StoreWriter::None,
        }
    }

    fn exists(&self, key: &Self::Key) -> Result<bool> {
        match self {
            Store::LruFsStore { store } => store.exists(key),
            Store::LfuFsStore { store } => store.exists(key),
            Store::FifoFsStore { store } => store.exists(key),
            Store::None => Ok(false),
        }
    }

    async fn lookup(&self, key: &Self::Key) -> Result<Option<Self::Value>> {
        match self {
            Store::LruFsStore { store } => store.lookup(key).await,
            Store::LfuFsStore { store } => store.lookup(key).await,
            Store::FifoFsStore { store } => store.lookup(key).await,
            Store::None => Ok(None),
        }
    }

    fn remove(&self, key: &Self::Key) -> Result<bool> {
        match self {
            Store::LruFsStore { store } => store.remove(key),
            Store::LfuFsStore { store } => store.remove(key),
            Store::FifoFsStore { store } => store.remove(key),
            Store::None => Ok(false),
        }
    }

    fn clear(&self) -> Result<()> {
        match self {
            Store::LruFsStore { store } => store.clear(),
            Store::LfuFsStore { store } => store.clear(),
            Store::FifoFsStore { store } => store.clear(),
            Store::None => Ok(()),
        }
    }
}
