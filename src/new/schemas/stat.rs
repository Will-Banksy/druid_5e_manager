use druid::Data;

#[derive(Clone, Data)]
pub enum Stat {
	Int(i32),
	UInt(u32),
	Float(f32)
}

// NOTE: Could completely genericisize everything, using this Attribute enumm where each data item would just be a list of Attributes
// But I'm not sure that'd be 1. very ergonomic or 2. very performant
// I could perhaps have every data item able to hold extra data in the form of a list of Attributes
// pub enum Attribute {
// 	Int(i32),
// 	UInt(u32),
// 	Float(f32),
// 	String(String)
// }