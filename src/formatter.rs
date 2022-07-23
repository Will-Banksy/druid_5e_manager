use std::{fmt::Display, str::FromStr, marker::PhantomData};

use druid::text::{Selection, Formatter, Validation, ValidationError};
use num::Zero;

pub struct NumberFormatter {}

pub struct RefNumberFormatter<T> {
	_marker: PhantomData<T>
}

impl NumberFormatter {
	pub fn new() -> Self {
		NumberFormatter {}
	}
}

impl<T> Formatter<T> for NumberFormatter where T: Display + FromStr + Zero, <T as FromStr>::Err: std::error::Error + 'static {
	fn format(&self, value: &T) -> String {
		format!("{}", value)
	}

	fn validate_partial_input(&self, input: &str, _sel: &Selection) -> Validation {
		match input.parse::<T>() {
			Ok(_) => Validation::success(),
			Err(e) => {
				if input.is_empty() {
					return Validation::success();
				}
				Validation::failure(e)
			}
		}
	}

	fn value(&self, input: &str) -> Result<T, ValidationError> {
		match input.parse::<T>() {
			Ok(value) => Ok(value),
			Err(e) => {
				if input.is_empty() {
					return Ok(T::zero());
				}
				Err(ValidationError::new(e))
			}
		}
	}
}

impl<T> RefNumberFormatter<T> {
	pub fn new() -> Self {
		RefNumberFormatter {
			_marker: PhantomData
		}
	}
}

impl<R, T> Formatter<R> for RefNumberFormatter<T> where R: AsRef<T> + From<T>, T: Display + FromStr + Zero, <T as FromStr>::Err: std::error::Error + 'static {
	fn format(&self, value: &R) -> String {
		format!("{}", value.as_ref())
	}

	fn validate_partial_input(&self, input: &str, _sel: &Selection) -> Validation {
		match input.parse::<T>() {
			Ok(_) => Validation::success(),
			Err(e) => {
				if input.is_empty() {
					return Validation::success();
				}
				Validation::failure(e)
			}
		}
    }

	fn value(&self, input: &str) -> Result<R, ValidationError> {
		match input.parse::<T>() {
			Ok(value) => { return Ok(R::from(value)); },
			Err(e) => {
				if input.is_empty() {
					return Ok(R::from(T::zero()));
				}
				Err(ValidationError::new(e))
			}
		}
    }
}
