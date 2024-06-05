use std::hash::Hash;

use bitflags::bitflags;
use deserr::Deserr;
use enum_iterator::Sequence;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Serialize, Deserialize, Debug, Eq, PartialEq, Hash, Sequence, Deserr)]
#[repr(u8)]
pub enum Action {
    #[serde(rename = "*")]
    #[deserr(rename = "*")]
    All = 0,
    #[serde(rename = "search")]
    #[deserr(rename = "search")]
    Search,
    #[serde(rename = "documents.*")]
    #[deserr(rename = "documents.*")]
    DocumentsAll,
    #[serde(rename = "documents.add")]
    #[deserr(rename = "documents.add")]
    DocumentsAdd,
    #[serde(rename = "documents.get")]
    #[deserr(rename = "documents.get")]
    DocumentsGet,
    #[serde(rename = "documents.delete")]
    #[deserr(rename = "documents.delete")]
    DocumentsDelete,
    #[serde(rename = "indexes.*")]
    #[deserr(rename = "indexes.*")]
    IndexesAll,
    #[serde(rename = "indexes.create")]
    #[deserr(rename = "indexes.create")]
    IndexesAdd,
    #[serde(rename = "indexes.get")]
    #[deserr(rename = "indexes.get")]
    IndexesGet,
    #[serde(rename = "indexes.update")]
    #[deserr(rename = "indexes.update")]
    IndexesUpdate,
    #[serde(rename = "indexes.delete")]
    #[deserr(rename = "indexes.delete")]
    IndexesDelete,
    #[serde(rename = "indexes.swap")]
    #[deserr(rename = "indexes.swap")]
    IndexesSwap,
    #[serde(rename = "tasks.*")]
    #[deserr(rename = "tasks.*")]
    TasksAll,
    #[serde(rename = "tasks.cancel")]
    #[deserr(rename = "tasks.cancel")]
    TasksCancel,
    #[serde(rename = "tasks.delete")]
    #[deserr(rename = "tasks.delete")]
    TasksDelete,
    #[serde(rename = "tasks.get")]
    #[deserr(rename = "tasks.get")]
    TasksGet,
    #[serde(rename = "settings.*")]
    #[deserr(rename = "settings.*")]
    SettingsAll,
    #[serde(rename = "settings.get")]
    #[deserr(rename = "settings.get")]
    SettingsGet,
    #[serde(rename = "settings.update")]
    #[deserr(rename = "settings.update")]
    SettingsUpdate,
    #[serde(rename = "stats.*")]
    #[deserr(rename = "stats.*")]
    StatsAll,
    #[serde(rename = "stats.get")]
    #[deserr(rename = "stats.get")]
    StatsGet,
    #[serde(rename = "metrics.*")]
    #[deserr(rename = "metrics.*")]
    MetricsAll,
    #[serde(rename = "metrics.get")]
    #[deserr(rename = "metrics.get")]
    MetricsGet,
    #[serde(rename = "dumps.*")]
    #[deserr(rename = "dumps.*")]
    DumpsAll,
    #[serde(rename = "dumps.create")]
    #[deserr(rename = "dumps.create")]
    DumpsCreate,
    #[serde(rename = "snapshots.*")]
    #[deserr(rename = "snapshots.*")]
    SnapshotsAll,
    #[serde(rename = "snapshots.create")]
    #[deserr(rename = "snapshots.create")]
    SnapshotsCreate,
    #[serde(rename = "version")]
    #[deserr(rename = "version")]
    Version,
    #[serde(rename = "keys.create")]
    #[deserr(rename = "keys.create")]
    KeysAdd,
    #[serde(rename = "keys.get")]
    #[deserr(rename = "keys.get")]
    KeysGet,
    #[serde(rename = "keys.update")]
    #[deserr(rename = "keys.update")]
    KeysUpdate,
    #[serde(rename = "keys.delete")]
    #[deserr(rename = "keys.delete")]
    KeysDelete,
    #[serde(rename = "experimental.get")]
    #[deserr(rename = "experimental.get")]
    ExperimentalFeaturesGet,
    #[serde(rename = "experimental.update")]
    #[deserr(rename = "experimental.update")]
    ExperimentalFeaturesUpdate,
}

impl Action {
    pub const fn from_repr(repr: u8) -> Option<Self> {
        use actions::*;
        match repr {
            ALL => Some(Self::All),
            SEARCH => Some(Self::Search),
            DOCUMENTS_ALL => Some(Self::DocumentsAll),
            DOCUMENTS_ADD => Some(Self::DocumentsAdd),
            DOCUMENTS_GET => Some(Self::DocumentsGet),
            DOCUMENTS_DELETE => Some(Self::DocumentsDelete),
            INDEXES_ALL => Some(Self::IndexesAll),
            INDEXES_CREATE => Some(Self::IndexesAdd),
            INDEXES_GET => Some(Self::IndexesGet),
            INDEXES_UPDATE => Some(Self::IndexesUpdate),
            INDEXES_DELETE => Some(Self::IndexesDelete),
            INDEXES_SWAP => Some(Self::IndexesSwap),
            TASKS_ALL => Some(Self::TasksAll),
            TASKS_CANCEL => Some(Self::TasksCancel),
            TASKS_DELETE => Some(Self::TasksDelete),
            TASKS_GET => Some(Self::TasksGet),
            SETTINGS_ALL => Some(Self::SettingsAll),
            SETTINGS_GET => Some(Self::SettingsGet),
            SETTINGS_UPDATE => Some(Self::SettingsUpdate),
            STATS_ALL => Some(Self::StatsAll),
            STATS_GET => Some(Self::StatsGet),
            METRICS_ALL => Some(Self::MetricsAll),
            METRICS_GET => Some(Self::MetricsGet),
            DUMPS_ALL => Some(Self::DumpsAll),
            DUMPS_CREATE => Some(Self::DumpsCreate),
            SNAPSHOTS_CREATE => Some(Self::SnapshotsCreate),
            VERSION => Some(Self::Version),
            KEYS_CREATE => Some(Self::KeysAdd),
            KEYS_GET => Some(Self::KeysGet),
            KEYS_UPDATE => Some(Self::KeysUpdate),
            KEYS_DELETE => Some(Self::KeysDelete),
            EXPERIMENTAL_FEATURES_GET => Some(Self::ExperimentalFeaturesGet),
            EXPERIMENTAL_FEATURES_UPDATE => Some(Self::ExperimentalFeaturesUpdate),
            _otherwise => None,
        }
    }

    pub const fn repr(&self) -> u8 {
        *self as u8
    }
}

pub mod actions {
    use super::Action::*;

    pub(crate) const ALL: u8 = All.repr();
    pub const SEARCH: u8 = Search.repr();
    pub const DOCUMENTS_ALL: u8 = DocumentsAll.repr();
    pub const DOCUMENTS_ADD: u8 = DocumentsAdd.repr();
    pub const DOCUMENTS_GET: u8 = DocumentsGet.repr();
    pub const DOCUMENTS_DELETE: u8 = DocumentsDelete.repr();
    pub const INDEXES_ALL: u8 = IndexesAll.repr();
    pub const INDEXES_CREATE: u8 = IndexesAdd.repr();
    pub const INDEXES_GET: u8 = IndexesGet.repr();
    pub const INDEXES_UPDATE: u8 = IndexesUpdate.repr();
    pub const INDEXES_DELETE: u8 = IndexesDelete.repr();
    pub const INDEXES_SWAP: u8 = IndexesSwap.repr();
    pub const TASKS_ALL: u8 = TasksAll.repr();
    pub const TASKS_CANCEL: u8 = TasksCancel.repr();
    pub const TASKS_DELETE: u8 = TasksDelete.repr();
    pub const TASKS_GET: u8 = TasksGet.repr();
    pub const SETTINGS_ALL: u8 = SettingsAll.repr();
    pub const SETTINGS_GET: u8 = SettingsGet.repr();
    pub const SETTINGS_UPDATE: u8 = SettingsUpdate.repr();
    pub const STATS_ALL: u8 = StatsAll.repr();
    pub const STATS_GET: u8 = StatsGet.repr();
    pub const METRICS_ALL: u8 = MetricsAll.repr();
    pub const METRICS_GET: u8 = MetricsGet.repr();
    pub const DUMPS_ALL: u8 = DumpsAll.repr();
    pub const DUMPS_CREATE: u8 = DumpsCreate.repr();
    pub const SNAPSHOTS_CREATE: u8 = SnapshotsCreate.repr();
    pub const VERSION: u8 = Version.repr();
    pub const KEYS_CREATE: u8 = KeysAdd.repr();
    pub const KEYS_GET: u8 = KeysGet.repr();
    pub const KEYS_UPDATE: u8 = KeysUpdate.repr();
    pub const KEYS_DELETE: u8 = KeysDelete.repr();
    pub const EXPERIMENTAL_FEATURES_GET: u8 = ExperimentalFeaturesGet.repr();
    pub const EXPERIMENTAL_FEATURES_UPDATE: u8 = ExperimentalFeaturesUpdate.repr();
}

#[derive(Copy, Clone, Serialize, Deserialize, Debug, Eq, PartialEq, Hash)]
pub struct ActionFlags(u64);

bitflags! {
    impl ActionFlags:u64{
        const Search = 1<<0;
        const DocumentsAll = 1 << 1;
        const DocumentsAdd = 1 << 2;
        const DocumentsGet = 1 << 3;
        const DocumentsDelete = 1 << 4;
        const IndexesAll = 1 << 5;
        const IndexesAdd = 1 << 6;
        const IndexesGet = 1 << 7;
        const IndexesUpdate = 1 << 8;
        const IndexesDelete = 1 << 9;
        const IndexesSwap = 1 << 10;
        const TasksAll = 1 << 11;
        const TasksCancel = 1 << 12;
        const TasksDelete = 1 << 13;
        const TasksGet = 1 << 15;
        const SettingsAll = 1 << 16;
        const SettingsGet = 1 << 17;
        const SettingsUpdate = 1 << 18;
        const StatsAll = 1 << 19;
        const StatsGet = 1 << 20;
        const MetricsAll = 1 << 21;
        const MetricsGet = 1 << 22;
        const DumpsAll = 1 << 23;
        const DumpsCreate = 1 << 24;
        const SnapshotsAll = 1 << 25;
        const SnapshotsCreate = 1 << 26;
        const Version = 1 << 27;
        const KeysAdd = 1 << 28;
        const KeysGet = 1 << 29;
        const KeysUpdate = 1 << 30;
        const KeysDelete = 1 << 31;
        const ExperimentalFeaturesGet = 1 << 32;
        const ExperimentalFeaturesUpdate = 1 << 33;

        const _ = !0;
        const All = 1 << 63;
    }
}
