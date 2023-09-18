use druid::Data;
use druid_widget_nursery::prism::Prism;

#[derive(Clone, PartialEq, Data, Prism)]
pub enum ValueAttribute {
	Int64(i64),
	UInt64(u64),
	Float64(f64)
}

impl From<i64> for ValueAttribute {
    fn from(value: i64) -> Self {
        Self::Int64(value)
    }
}

impl From<u64> for ValueAttribute {
	fn from(value: u64) -> Self {
		Self::UInt64(value)
	}
}

impl From<f64> for ValueAttribute {
	fn from(value: f64) -> Self {
		ValueAttribute::Float64(value)
	}
}