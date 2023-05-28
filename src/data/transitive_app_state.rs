use druid::Data;

#[derive(Data, Clone)]
pub struct TransitiveAppState {
	show_overlay: bool
}