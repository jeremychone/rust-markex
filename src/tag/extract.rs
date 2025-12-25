//! Parser module for extracting tag elements and text fragments from input.

use super::support::parse_attribute;
use super::tag_elem::{Part, TagElem};
use super::tag_elem_ref_iter::{PartRef, TagElemRefIterator};
use serde::Serialize;

/// Result of extracting data and parts from input.
#[derive(Debug, Serialize, Clone, PartialEq, Default)]
pub struct ExtractedData {
	pub tag_names: Vec<String>,
	pub parts: Vec<Part>,
}

impl ExtractedData {
	/// Returns references to all `TagElem` items in the parsed data.
	pub fn tag_elems(&self) -> Vec<&TagElem> {
		self.parts
			.iter()
			.filter_map(|part| match part {
				Part::TagElem(elem) => Some(elem),
				Part::Text(_) => None,
			})
			.collect()
	}

	/// Consumes the parsed data and returns all `TagElem` items.
	pub fn into_tag_elems(self) -> Vec<TagElem> {
		self.parts
			.into_iter()
			.filter_map(|part| match part {
				Part::TagElem(elem) => Some(elem),
				Part::Text(_) => None,
			})
			.collect()
	}

	/// Returns references to all text strings in the parsed data.
	pub fn texts(&self) -> Vec<&String> {
		self.parts
			.iter()
			.filter_map(|part| match part {
				Part::Text(text) => Some(text),
				Part::TagElem(_) => None,
			})
			.collect()
	}

	/// Consumes the parsed data and returns all text strings.
	pub fn into_texts(self) -> Vec<String> {
		self.parts
			.into_iter()
			.filter_map(|part| match part {
				Part::Text(text) => Some(text),
				Part::TagElem(_) => None,
			})
			.collect()
	}

	/// Consumes the parsed data and returns the tag elements along with all text concatenated into a single string.
	pub fn into_with_extrude_content(self) -> (Vec<TagElem>, String) {
		let mut tag_elems = Vec::new();
		let mut text_content = String::new();

		for part in self.parts {
			match part {
				Part::TagElem(elem) => tag_elems.push(elem),
				Part::Text(text) => text_content.push_str(&text),
			}
		}

		(tag_elems, text_content)
	}
}

/// Parses the input string for the specified tag names.
///
/// # Arguments
///
/// * `input` - The string slice to parse.
/// * `tag_names` - A slice of tag names to search for (e.g., &["FILE", "DATA"]).
/// * `capture_text` - If true, includes `Part::Text` fragments in the result; otherwise, only `Part::TagElem` are included.
///
/// # Returns
///
/// A `ExtractedData` containing the extracted parts.
pub fn extract(input: &str, tag_names: &[&str], capture_text: bool) -> ExtractedData {
	let iter = TagElemRefIterator::new(input, tag_names);
	let mut parts = Vec::new();

	for part_ref in iter {
		match part_ref {
			PartRef::Text(text) => {
				if capture_text {
					parts.push(Part::Text(text.to_string()));
				}
			}
			PartRef::TagElemRef(tag_ref) => {
				parts.push(Part::TagElem(TagElem {
					tag: tag_ref.tag_name.to_string(),
					attrs: parse_attribute(tag_ref.attrs_raw),
					content: tag_ref.content.to_string(),
				}));
			}
		}
	}

	ExtractedData {
		tag_names: tag_names.iter().map(|s| s.to_string()).collect(),
		parts,
	}
}

// region:    --- Tests

#[path = "extract_tests.rs"]
#[cfg(test)]
mod tests;

// endregion: --- Tests
