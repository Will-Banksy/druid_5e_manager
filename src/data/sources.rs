mod srd;
mod source_utils;

use super::internal::InternalSource;

fn get_sources() -> Vec<InternalSource> { // TODO: Maybe make this return a future although the SRD stuff should be fast enough to not need to do that
	vec![
		srd::SrdSource::get_as_internal()
	]
}

pub trait Source {
	fn get_as_internal() -> InternalSource;
}