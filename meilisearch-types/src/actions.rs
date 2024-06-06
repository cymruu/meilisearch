use std::hash::Hash;

use bitflags::bitflags;
use deserr::Deserr;
use enum_iterator::Sequence;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Serialize, Deserialize, Debug, Eq, PartialEq, Hash, Sequence, Deserr)]
#[repr(u64)]
pub enum Action {
    #[serde(rename = "*")]
    #[deserr(rename = "*")]
    All = ActionFlags::All.bits(),
    #[serde(rename = "search")]
    #[deserr(rename = "search")]
    Search = ActionFlags::Search.bits(),
    #[serde(rename = "documents.*")]
    #[deserr(rename = "documents.*")]
    DocumentsAll = ActionFlags::DocumentsAll.bits(),
    #[serde(rename = "documents.add")]
    #[deserr(rename = "documents.add")]
    DocumentsAdd = ActionFlags::DocumentsAdd.bits(),
    #[serde(rename = "documents.get")]
    #[deserr(rename = "documents.get")]
    DocumentsGet = ActionFlags::DocumentsGet.bits(),
    #[serde(rename = "documents.delete")]
    #[deserr(rename = "documents.delete")]
    DocumentsDelete = ActionFlags::DocumentsDelete.bits(),
    #[serde(rename = "indexes.*")]
    #[deserr(rename = "indexes.*")]
    IndexesAll = ActionFlags::IndexesAll.bits(),
    #[serde(rename = "indexes.create")]
    #[deserr(rename = "indexes.create")]
    IndexesAdd = ActionFlags::IndexesAdd.bits(),
    #[serde(rename = "indexes.get")]
    #[deserr(rename = "indexes.get")]
    IndexesGet = ActionFlags::IndexesGet.bits(),
    #[serde(rename = "indexes.update")]
    #[deserr(rename = "indexes.update")]
    IndexesUpdate = ActionFlags::IndexesUpdate.bits(),
    #[serde(rename = "indexes.delete")]
    #[deserr(rename = "indexes.delete")]
    IndexesDelete = ActionFlags::IndexesDelete.bits(),
    #[serde(rename = "indexes.swap")]
    #[deserr(rename = "indexes.swap")]
    IndexesSwap = ActionFlags::IndexesSwap.bits(),
    #[serde(rename = "tasks.*")]
    #[deserr(rename = "tasks.*")]
    TasksAll = ActionFlags::TasksAll.bits(),
    #[serde(rename = "tasks.cancel")]
    #[deserr(rename = "tasks.cancel")]
    TasksCancel = ActionFlags::TasksCancel.bits(),
    #[serde(rename = "tasks.delete")]
    #[deserr(rename = "tasks.delete")]
    TasksDelete = ActionFlags::TasksDelete.bits(),
    #[serde(rename = "tasks.get")]
    #[deserr(rename = "tasks.get")]
    TasksGet = ActionFlags::TasksGet.bits(),
    #[serde(rename = "settings.*")]
    #[deserr(rename = "settings.*")]
    SettingsAll = ActionFlags::SettingsAll.bits(),
    #[serde(rename = "settings.get")]
    #[deserr(rename = "settings.get")]
    SettingsGet = ActionFlags::SettingsGet.bits(),
    #[serde(rename = "settings.update")]
    #[deserr(rename = "settings.update")]
    SettingsUpdate = ActionFlags::SettingsUpdate.bits(),
    #[serde(rename = "stats.*")]
    #[deserr(rename = "stats.*")]
    StatsAll = ActionFlags::StatsAll.bits(),
    #[serde(rename = "stats.get")]
    #[deserr(rename = "stats.get")]
    StatsGet = ActionFlags::StatsGet.bits(),
    #[serde(rename = "metrics.*")]
    #[deserr(rename = "metrics.*")]
    MetricsAll = ActionFlags::MetricsAll.bits(),
    #[serde(rename = "metrics.get")]
    #[deserr(rename = "metrics.get")]
    MetricsGet = ActionFlags::MetricsGet.bits(),
    #[serde(rename = "dumps.*")]
    #[deserr(rename = "dumps.*")]
    DumpsAll = ActionFlags::DumpsAll.bits(),
    #[serde(rename = "dumps.create")]
    #[deserr(rename = "dumps.create")]
    DumpsCreate = ActionFlags::DumpsCreate.bits(),
    #[serde(rename = "snapshots.*")]
    #[deserr(rename = "snapshots.*")]
    SnapshotsAll = ActionFlags::SnapshotsAll.bits(),
    #[serde(rename = "snapshots.create")]
    #[deserr(rename = "snapshots.create")]
    SnapshotsCreate = ActionFlags::SnapshotsCreate.bits(),
    #[serde(rename = "version")]
    #[deserr(rename = "version")]
    Version = ActionFlags::Version.bits(),
    #[serde(rename = "keys.create")]
    #[deserr(rename = "keys.create")]
    KeysAdd = ActionFlags::KeysAdd.bits(),
    #[serde(rename = "keys.get")]
    #[deserr(rename = "keys.get")]
    KeysGet = ActionFlags::KeysGet.bits(),
    #[serde(rename = "keys.update")]
    #[deserr(rename = "keys.update")]
    KeysUpdate = ActionFlags::KeysUpdate.bits(),
    #[serde(rename = "keys.delete")]
    #[deserr(rename = "keys.delete")]
    KeysDelete = ActionFlags::KeysDelete.bits(),
    #[serde(rename = "experimental.get")]
    #[deserr(rename = "experimental.get")]
    ExperimentalFeaturesGet = ActionFlags::ExperimentalFeaturesGet.bits(),
    #[serde(rename = "experimental.update")]
    #[deserr(rename = "experimental.update")]
    ExperimentalFeaturesUpdate = ActionFlags::ExperimentalFeaturesUpdate.bits(),
}

impl Action {
    pub const fn from_repr(repr: u64) -> Option<Self> {
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

    pub const fn repr(&self) -> u64 {
        *self as u64
    }
}

pub mod actions {
    use super::Action::*;

    pub(crate) const ALL: u64 = All.repr();
    pub const SEARCH: u64 = Search.repr();
    pub const DOCUMENTS_ALL: u64 = DocumentsAll.repr();
    pub const DOCUMENTS_ADD: u64 = DocumentsAdd.repr();
    pub const DOCUMENTS_GET: u64 = DocumentsGet.repr();
    pub const DOCUMENTS_DELETE: u64 = DocumentsDelete.repr();
    pub const INDEXES_ALL: u64 = IndexesAll.repr();
    pub const INDEXES_CREATE: u64 = IndexesAdd.repr();
    pub const INDEXES_GET: u64 = IndexesGet.repr();
    pub const INDEXES_UPDATE: u64 = IndexesUpdate.repr();
    pub const INDEXES_DELETE: u64 = IndexesDelete.repr();
    pub const INDEXES_SWAP: u64 = IndexesSwap.repr();
    pub const TASKS_ALL: u64 = TasksAll.repr();
    pub const TASKS_CANCEL: u64 = TasksCancel.repr();
    pub const TASKS_DELETE: u64 = TasksDelete.repr();
    pub const TASKS_GET: u64 = TasksGet.repr();
    pub const SETTINGS_ALL: u64 = SettingsAll.repr();
    pub const SETTINGS_GET: u64 = SettingsGet.repr();
    pub const SETTINGS_UPDATE: u64 = SettingsUpdate.repr();
    pub const STATS_ALL: u64 = StatsAll.repr();
    pub const STATS_GET: u64 = StatsGet.repr();
    pub const METRICS_ALL: u64 = MetricsAll.repr();
    pub const METRICS_GET: u64 = MetricsGet.repr();
    pub const DUMPS_ALL: u64 = DumpsAll.repr();
    pub const DUMPS_CREATE: u64 = DumpsCreate.repr();
    pub const SNAPSHOTS_CREATE: u64 = SnapshotsCreate.repr();
    pub const VERSION: u64 = Version.repr();
    pub const KEYS_CREATE: u64 = KeysAdd.repr();
    pub const KEYS_GET: u64 = KeysGet.repr();
    pub const KEYS_UPDATE: u64 = KeysUpdate.repr();
    pub const KEYS_DELETE: u64 = KeysDelete.repr();
    pub const EXPERIMENTAL_FEATURES_GET: u64 = ExperimentalFeaturesGet.repr();
    pub const EXPERIMENTAL_FEATURES_UPDATE: u64 = ExperimentalFeaturesUpdate.repr();
}

#[derive(Copy, Clone, Serialize, Deserialize, Debug, Eq, PartialEq, Hash)]
pub struct ActionFlags(u64);

bitflags! {
    impl ActionFlags:u64{
        const Search = 1 << 0;
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
