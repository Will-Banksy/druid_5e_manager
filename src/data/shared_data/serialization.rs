use serde::{Serialize, Deserialize};

use super::{SharedDataItem, SharedData, SHARED_MAP};

#[derive(Serialize, Deserialize)]
pub struct SerializableSharedData {
	uuid: u128,
	data: SharedDataItem
}

impl From<SerializableSharedData> for SharedData {
	fn from(sdata: SerializableSharedData) -> Self {
		SharedData::make(sdata.uuid, sdata.data)
	}
}

impl From<SharedData> for SerializableSharedData {
	fn from(data: SharedData) -> Self {
		SerializableSharedData { uuid: data.uuid, data: SHARED_MAP.read().unwrap().get(&data.uuid).unwrap().to_owned() }
	}
}