// INFO: This is kinda unused atm cause druid doesn't seem to have image buttons or a way of displaying a SVG in a button :/

use lazy_static::lazy_static;

pub const ASSET_SVG_BIN: &[u8; 1208] = include_bytes!("../assets/bin.svg");
pub const ASSET_IMG_BIN: &[u8; 235] = include_bytes!("../assets/bin.png");

lazy_static! {
	pub static ref ASSETIMAGE_IMG_BIN: Vec<u8> = {
		let img = image::load_from_memory_with_format(ASSET_IMG_BIN, image::ImageFormat::Png).unwrap();
		img.into_bytes()
	};
}