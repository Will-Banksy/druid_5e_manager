mod srd;
mod source_utils;

use crate::data::internal::{SourceCategory, SourceContentCollection, SourceContentType};

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
			content: im::vector![
				SourceContentCollection::empty(SourceContentType::ArmourType),
				SourceContentCollection::empty(SourceContentType::FeatType)
			]
		},
		InternalSource {
			name:"A very long source name that really goes on forever it's stupid".into(),
			category: SourceCategory::Core,
			content: im::vector![
				SourceContentCollection::empty(SourceContentType::ArmourType),
				SourceContentCollection::empty(SourceContentType::FeatType)
			]
		},
		InternalSource {
			name:"A very long source name that really goes on forever it's stupid 2: Electric Boogaloo".into(),
			category: SourceCategory::Supplements,
			content: im::vector![
				SourceContentCollection::empty(SourceContentType::ArmourType),
				SourceContentCollection::empty(SourceContentType::FeatType)
			]
		},
		InternalSource {
			name:"Source with lots of armours".into(),
			category: SourceCategory::Supplements,
			content: im::vector![
				SourceContentCollection::empty(SourceContentType::ArmourType),
				SourceContentCollection::empty(SourceContentType::ArmourType),
				SourceContentCollection::empty(SourceContentType::ArmourType),
				SourceContentCollection::empty(SourceContentType::ArmourType),
				SourceContentCollection::empty(SourceContentType::ArmourType),
				SourceContentCollection::empty(SourceContentType::ArmourType),
				SourceContentCollection::empty(SourceContentType::FeatType)
			]
		},
		InternalSource {
			name:"Source with like nothing".into(),
			category: SourceCategory::Supplements,
			content: im::vector![
				SourceContentCollection::empty(SourceContentType::FeatType)
			]
		},
		InternalSource {
			name:"Source with like nothing".into(),
			category: SourceCategory::Supplements,
			content: im::vector![
			]
		}
	]
}

pub trait Source {
	fn get_as_internal() -> InternalSource;
}