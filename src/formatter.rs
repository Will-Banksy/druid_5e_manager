use std::{fmt::Display, str::FromStr};

use druid::text::{Selection, Formatter, Validation, ValidationError};
use num::Zero;

pub struct NumberFormatter {
	unit: Option<&'static str>
}

impl NumberFormatter {
	pub fn new() -> Self {
		NumberFormatter {
			unit: None
		}
	}

	pub fn set_unit(&mut self, unit: &'static str) {
		self.unit = Some(unit);
	}

	pub fn with_unit(mut self, unit: &'static str) -> Self {
		self.set_unit(unit);
		self
	}
}

impl<T> Formatter<T> for NumberFormatter where T: Display + FromStr + Zero, <T as FromStr>::Err: std::error::Error + 'static {
	fn format(&self, value: &T) -> String {
		if let Some(u) = self.unit {
			format!("{} {}", value, u)
		} else {
			format!("{}", value)
		}
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

	fn format_for_editing(&self, value: &T) -> String {
		format!("{}", value)
	}
}
