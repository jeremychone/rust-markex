use crate::tag::{PartRef, TagElemRef};

/// Result of extracting data and parts from input as references.
#[derive(Debug, PartialEq, Default)]
pub struct PartsRef<'a> {
	pub(crate) parts: Vec<PartRef<'a>>,
}

impl<'a> PartsRef<'a> {
	pub fn parts(&self) -> &Vec<PartRef<'a>> {
		&self.parts
	}

	pub fn into_parts(self) -> Vec<PartRef<'a>> {
		self.parts
	}

	/// Returns the unique tag names found in the parts.
	pub fn tag_names(&self) -> Vec<&str> {
		let mut names = Vec::new();
		for part in &self.parts {
			if let PartRef::TagElemRef(elem) = part {
				if !names.contains(&elem.tag_name) {
					names.push(elem.tag_name);
				}
			}
		}
		names
	}

	pub fn iter(&self) -> std::slice::Iter<'_, PartRef<'a>> {
		self.parts.iter()
	}

	/// Returns references to all `TagElemRef` items in the parsed data.
	pub fn tag_elems(&self) -> Vec<&TagElemRef<'a>> {
		self.parts
			.iter()
			.filter_map(|part| match part {
				PartRef::TagElemRef(elem) => Some(elem),
				PartRef::Text(_) => None,
			})
			.collect()
	}

	/// Returns all text strings in the parsed data.
	pub fn texts(&self) -> Vec<&'a str> {
		self.parts
			.iter()
			.filter_map(|part| match part {
				PartRef::Text(text) => Some(*text),
				PartRef::TagElemRef(_) => None,
			})
			.collect()
	}
}

impl<'a> IntoIterator for PartsRef<'a> {
	type Item = PartRef<'a>;
	type IntoIter = std::vec::IntoIter<Self::Item>;

	fn into_iter(self) -> Self::IntoIter {
		self.parts.into_iter()
	}
}

impl<'a, 'b> IntoIterator for &'b PartsRef<'a> {
	type Item = &'b PartRef<'a>;
	type IntoIter = std::slice::Iter<'b, PartRef<'a>>;

	fn into_iter(self) -> Self::IntoIter {
		self.parts.iter()
	}
}

impl<'a> From<PartsRef<'a>> for Vec<PartRef<'a>> {
	fn from(val: PartsRef<'a>) -> Self {
		val.parts
	}
}
