pub trait StringCasingExt {
	type Owned;

	fn to_ascii_capitalised(&self) -> Self::Owned;
}

macro_rules! impl_trait_for_strs {
	($str_type: ty) => {
		impl StringCasingExt for $str_type {
			type Owned = String;

			fn to_ascii_capitalised(&self) -> Self::Owned {
				let mut res = String::with_capacity(self.len());
				let mut prev_c: Option<char> = None;

				for c in self.chars() {
					res.push(
						if let Some(pc) = prev_c {
							let ret = if pc.is_ascii_whitespace() {
								c.to_ascii_uppercase()
							} else {
								c.to_ascii_lowercase()
							};

							prev_c = Some(c);

							ret
						} else {
							let ret = c.to_ascii_uppercase();

							prev_c = Some(c);

							ret
						}
					)
				}

				res
			}
		}
	};
}

impl_trait_for_strs!(str);
impl_trait_for_strs!(String);

// impl StringCasingExt for str {
// 	type Owned = String;

// 	fn to_ascii_capitalised(&self) -> Self::Owned {
//         let mut res = String::with_capacity(self.len());
// 		let mut prev_c: Option<char> = None;

// 		for c in self.chars() {
// 			res.push(
// 				if let Some(pc) = prev_c {
// 					let ret = if pc.is_ascii_whitespace() {
// 						c.to_ascii_uppercase()
// 					} else {
// 						c.to_ascii_lowercase()
// 					};

// 					prev_c = Some(c);

// 					ret
// 				} else {
// 					let ret = c.to_ascii_uppercase();

// 					prev_c = Some(c);

// 					ret
// 				}
// 			)
// 		}

// 		res
//     }
// }