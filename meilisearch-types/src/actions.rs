use std::hash::Hash;

use bitflags::bitflags;
use deserr::{take_cf_content, DeserializeError, Deserr, ValueKind};
use enum_iterator::Sequence;
use serde::{Deserialize, Serialize};

//
// #[derive(Copy, Clone, Serialize, Deserialize, Debug, Eq, PartialEq, Hash, Sequence, Deserr)]
// #[repr(u64)]
// pub enum Action {
//     #[serde(rename = "*")]
//     #[deserr(rename = "*")]
//     All = ActionFlags::All.bits(),
//     #[serde(rename = "search")]
//     #[deserr(rename = "search")]
//     Search = ActionFlags::Search.bits(),
//     #[serde(rename = "documents.*")]
//     #[deserr(rename = "documents.*")]
//     DocumentsAll = ActionFlags::DocumentsAll.bits(),
//     #[serde(rename = "documents.add")]
//     #[deserr(rename = "documents.add")]
//     DocumentsAdd = ActionFlags::DocumentsAdd.bits(),
//     #[serde(rename = "documents.get")]
//     #[deserr(rename = "documents.get")]
//     DocumentsGet = ActionFlags::DocumentsGet.bits(),
//     #[serde(rename = "documents.delete")]
//     #[deserr(rename = "documents.delete")]
//     DocumentsDelete = ActionFlags::DocumentsDelete.bits(),
//     #[serde(rename = "indexes.*")]
//     #[deserr(rename = "indexes.*")]
//     IndexesAll = ActionFlags::IndexesAll.bits(),
//     #[serde(rename = "indexes.create")]
//     #[deserr(rename = "indexes.create")]
//     IndexesAdd = ActionFlags::IndexesAdd.bits(),
//     #[serde(rename = "indexes.get")]
//     #[deserr(rename = "indexes.get")]
//     IndexesGet = ActionFlags::IndexesGet.bits(),
//     #[serde(rename = "indexes.update")]
//     #[deserr(rename = "indexes.update")]
//     IndexesUpdate = ActionFlags::IndexesUpdate.bits(),
//     #[serde(rename = "indexes.delete")]
//     #[deserr(rename = "indexes.delete")]
//     IndexesDelete = ActionFlags::IndexesDelete.bits(),
//     #[serde(rename = "indexes.swap")]
//     #[deserr(rename = "indexes.swap")]
//     IndexesSwap = ActionFlags::IndexesSwap.bits(),
//     #[serde(rename = "tasks.*")]
//     #[deserr(rename = "tasks.*")]
//     TasksAll = ActionFlags::TasksAll.bits(),
//     #[serde(rename = "tasks.cancel")]
//     #[deserr(rename = "tasks.cancel")]
//     TasksCancel = ActionFlags::TasksCancel.bits(),
//     #[serde(rename = "tasks.delete")]
//     #[deserr(rename = "tasks.delete")]
//     TasksDelete = ActionFlags::TasksDelete.bits(),
//     #[serde(rename = "tasks.get")]
//     #[deserr(rename = "tasks.get")]
//     TasksGet = ActionFlags::TasksGet.bits(),
//     #[serde(rename = "settings.*")]
//     #[deserr(rename = "settings.*")]
//     SettingsAll = ActionFlags::SettingsAll.bits(),
//     #[serde(rename = "settings.get")]
//     #[deserr(rename = "settings.get")]
//     SettingsGet = ActionFlags::SettingsGet.bits(),
//     #[serde(rename = "settings.update")]
//     #[deserr(rename = "settings.update")]
//     SettingsUpdate = ActionFlags::SettingsUpdate.bits(),
//     #[serde(rename = "stats.*")]
//     #[deserr(rename = "stats.*")]
//     StatsAll = ActionFlags::StatsAll.bits(),
//     #[serde(rename = "stats.get")]
//     #[deserr(rename = "stats.get")]
//     StatsGet = ActionFlags::StatsGet.bits(),
//     #[serde(rename = "metrics.*")]
//     #[deserr(rename = "metrics.*")]
//     MetricsAll = ActionFlags::MetricsAll.bits(),
//     #[serde(rename = "metrics.get")]
//     #[deserr(rename = "metrics.get")]
//     MetricsGet = ActionFlags::MetricsGet.bits(),
//     #[serde(rename = "dumps.*")]
//     #[deserr(rename = "dumps.*")]
//     DumpsAll = ActionFlags::DumpsAll.bits(),
//     #[serde(rename = "dumps.create")]
//     #[deserr(rename = "dumps.create")]
//     DumpsCreate = ActionFlags::DumpsCreate.bits(),
//     #[serde(rename = "snapshots.*")]
//     #[deserr(rename = "snapshots.*")]
//     SnapshotsAll = ActionFlags::SnapshotsAll.bits(),
//     #[serde(rename = "snapshots.create")]
//     #[deserr(rename = "snapshots.create")]
//     SnapshotsCreate = ActionFlags::SnapshotsCreate.bits(),
//     #[serde(rename = "version")]
//     #[deserr(rename = "version")]
//     Version = ActionFlags::Version.bits(),
//     #[serde(rename = "keys.create")]
//     #[deserr(rename = "keys.create")]
//     KeysAdd = ActionFlags::KeysAdd.bits(),
//     #[serde(rename = "keys.get")]
//     #[deserr(rename = "keys.get")]
//     KeysGet = ActionFlags::KeysGet.bits(),
//     #[serde(rename = "keys.update")]
//     #[deserr(rename = "keys.update")]
//     KeysUpdate = ActionFlags::KeysUpdate.bits(),
//     #[serde(rename = "keys.delete")]
//     #[deserr(rename = "keys.delete")]
//     KeysDelete = ActionFlags::KeysDelete.bits(),
// #[serde(rename = "experimental.get")]
// #[deserr(rename = "experimental.get")]
// ExperimentalFeaturesGet = ActionFlags::ExperimentalFeaturesGet.bits(),
// #[serde(rename = "experimental.update")]
// #[deserr(rename = "experimental.update")]
// ExperimentalFeaturesUpdate = ActionFlags::ExperimentalFeaturesUpdate.bits(),
// }
//

impl Action {
    pub const fn from_repr(repr: u64) -> Option<Self> {
        Self::from_bits(repr)
    }

    pub const fn repr(&self) -> u64 {
        self.0 as u64
    }
}

#[derive(Copy, Clone, Serialize, Deserialize, Debug, Eq, PartialEq, Hash)]
pub struct Action(u64);

impl<E: DeserializeError> Deserr<E> for Action {
    fn deserialize_from_value<V: deserr::IntoValue>(
        value: deserr::Value<V>,
        location: deserr::ValuePointerRef,
    ) -> Result<Self, E> {
        match value {
            deserr::Value::Integer(repr) => Ok(Self::from_repr(repr).unwrap()),
            _ => Err(take_cf_content(E::error(
                None,
                deserr::ErrorKind::IncorrectValueKind {
                    actual: value,
                    accepted: &[ValueKind::Integer],
                },
                location,
            ))),
        }
    }
}

bitflags! {
    impl Action:u64{
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
