//! Iterator to extract structured `TagElem`s including content and parsed attributes.

use super::TagElem;
use super::TagElemRefIterator;
use super::attrs_parser::parse_attribute;
use super::tag_elem_ref_iter::PartRef;

/// Iterator that yields `TagElem` instances found within a text based on specific tag names.
/// It uses `TagElemRefIterator` internally to find tag boundaries.
pub struct TagElemIter<'a> {
	tag_content_iter: TagElemRefIterator<'a>,
}

impl<'a> TagElemIter<'a> {
	/// Creates a new `TagElemIter`.
	///
	/// # Arguments
	///
	/// * `input` - The string slice to search within.
	/// * `tag_names` - A slice of tag names to search for (e.g., &["FILE", "DATA"]).
	pub fn new(input: &'a str, tag_names: &[&'a str]) -> Self {
		let tag_names_vec: Vec<&'a str> = tag_names.to_vec();
		let tag_content_iter = TagElemRefIterator::new(input, &tag_names_vec);

		Self { tag_content_iter }
	}

	/// Creates a new `TagElemIter` configured to search for a single tag name.
	///
	/// This is a convenience wrapper around `Self::new`.
	///
	/// # Arguments
	///
	/// * `input` - The string slice to search within.
	/// * `tag_name` - The name of the tag to search for (e.g., "FILE").
	pub fn new_single_tag(input: &'a str, tag_name: &'a str) -> Self {
		Self::new(input, &[tag_name])
	}
}

impl Iterator for TagElemIter<'_> {
	type Item = TagElem;

	fn next(&mut self) -> Option<Self::Item> {
		loop {
			match self.tag_content_iter.next()? {
				PartRef::Text(_) => {
					// Skip text fragments, only yield TagElems
					continue;
				}
				PartRef::TagElemRef(tag_content) => {
					return Some(TagElem {
						tag: tag_content.tag_name.to_string(),
						attrs: parse_attribute(tag_content.attrs_raw),
						content: tag_content.content.to_string(),
					});
				}
			}
		}
	}
}

// region:    --- Tests

#[path = "tag_elem_iter_tests.rs"]
#[cfg(test)]
mod tests;

// endregion: --- Tests
