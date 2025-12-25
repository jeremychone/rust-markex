use crate::tag::{PartRef, TagElemRef};

/// Result of extracting data and parts from input as references.
#[derive(Debug, PartialEq)]
pub struct PartsRef<'a> {
	pub tag_names: Vec<String>,
	pub parts: Vec<PartRef<'a>>,
}

impl<'a> PartsRef<'a> {
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
