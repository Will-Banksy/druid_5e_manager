mod srd;
mod source_utils;

use crate::data::internal::SourceCategory;

use super::internal::InternalSource;

pub fn get_sources() -> Vec<InternalSource> { // TODO: Maybe make this return a future although the SRD stuff should be fast enough to not need to do that
	vec![
		srd::SrdSource::get_as_internal(),
		srd::SrdSource::get_as_internal(),
		srd::SrdSource::get_as_internal(),
		srd::SrdSource::get_as_internal(),
		srd::SrdSource::get_as_internal(),
		srd::SrdSource::get_as_internal(),
		InternalSource {
			name:"A source".into(),
			category: SourceCategory::Core,
			armours: im::Vector::new(),
			feats: im::Vector::new()
		},
		InternalSource {
			name:"A very long source name that really goes on forever it's stupid".into(),
			category: SourceCategory::Core,
			armours: im::Vector::new(),
			feats: im::Vector::new()
		},
		InternalSource {
			name:"A very long source name that really goes on forever it's stupid 2: Electric Boogaloo".into(),
			category: SourceCategory::Supplements,
			armours: im::Vector::new(),
			feats: im::Vector::new()
		}
	]
}

pub trait Source {
	fn get_as_internal() -> InternalSource;
}