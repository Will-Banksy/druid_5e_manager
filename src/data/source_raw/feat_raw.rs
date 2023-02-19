use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct FeatRaw {
	name: String,
	prerequisite: String,
	desc: String
}

#[cfg(test)]
#[test]
fn test_feat_deserialization() {
	let feat_json = include_str!("../../../assets/sources/wotc_srd/feats.json");
	let feat: Vec<FeatRaw> = serde_json::from_str(feat_json).unwrap();
	println!("FeatRaw Array: {:?}", feat);
}