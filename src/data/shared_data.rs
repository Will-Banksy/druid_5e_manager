mod serialization;
pub mod lens;

use std::{sync::{RwLock}, fmt::Display};
use druid::Data;
use im::HashMap;
use lazy_static::lazy_static;
use serde::{Serialize, Deserialize};
use uuid::Uuid;

lazy_static! {
	// TODO: Re-evaluate - Could this type be a cause of bugs due to threading? HashMap is threadsafe, but SharedDataItem is not... Does it matter?
	static ref SHARED_MAP: RwLock<HashMap<u128, SharedDataItem>> = RwLock::new(HashMap::new());
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, variantly::Variantly)]
pub enum SharedDataItem {
	U8(u8),
	U16(u16),
	U32(u32),
	U64(u64),
	U128(u128),
	I8(i8),
	I16(i16),
	I32(i32),
	I64(i64),
	I128(i128),
	String(String)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(into = "serialization::SerializableSharedData", from = "serialization::SerializableSharedData")]
pub struct SharedData {
	uuid: u128,
	local_copy: SharedDataItem
}

impl Display for SharedDataItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SharedDataItem::U8(n) => write!(f, "{}", n),
            SharedDataItem::U16(n) => write!(f, "{}", n),
            SharedDataItem::U32(n) => write!(f, "{}", n),
            SharedDataItem::U64(n) => write!(f, "{}", n),
            SharedDataItem::U128(n) => write!(f, "{}", n),
            SharedDataItem::I8(n) => write!(f, "{}", n),
            SharedDataItem::I16(n) => write!(f, "{}", n),
            SharedDataItem::I32(n) => write!(f, "{}", n),
            SharedDataItem::I64(n) => write!(f, "{}", n),
            SharedDataItem::I128(n) => write!(f, "{}", n),
            SharedDataItem::String(n) => write!(f, "{}", n),
        }
    }
}

/// Maybe have a way of indicating the type of data?
impl SharedData {
	pub fn new(data: SharedDataItem) -> Self {
		let uuid = Uuid::new_v4().as_u128();
		let local_copy = data.clone();
		SHARED_MAP.write().unwrap().insert(uuid, data);
		SharedData { uuid, local_copy }
	}

	/// Returns none if there is no entry in the shared map for this uuid
	pub fn from_uuid(uuid: u128) -> Option<Self> {
		if SHARED_MAP.read().unwrap().contains_key(&uuid) {
			Some(SharedData { uuid, local_copy: SHARED_MAP.read().unwrap().get(&uuid).unwrap().clone() })
		} else {
			None
		}
	}

	pub fn make(uuid: u128, data: SharedDataItem) -> Self {
		let local_copy = data.clone();
		SHARED_MAP.write().unwrap().insert(uuid, data);
		SharedData { uuid, local_copy }
	}

	pub fn uuid(&self) -> u128 {
		self.uuid
	}

	pub fn item(&self) -> SharedDataItem {
		SHARED_MAP.read().unwrap().get(&self.uuid).unwrap().clone()
	}

	pub fn local_copy(&self) -> &SharedDataItem {
		&self.local_copy
	}

	pub fn backup(&mut self, data: SharedDataItem) {
		self.local_copy = data;
	}
}

impl Display for SharedData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let val = self.item();
		write!(f, "{}", val)
    }
}

impl Data for SharedData {
    fn same(&self, other: &Self) -> bool {
		self.uuid == other.uuid && *self.local_copy() == self.item() && *other.local_copy() == other.item()
    }
}
