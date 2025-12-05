//! Iterator to extract structured `TagElem`s including content and parsed attributes.

use super::{Part, TagElem, TagElemRefIterator};
use super::attrs_parser::parse_attribute;
use super::tag_elem_ref_iter::PartRef;

/// Iterator that yields owned `Part` instances (`Text` or `TagElem`), found within a text
/// based on specific tag names.
/// It consumes the referenced elements from `TagElemRefIterator` and converts them to owned types.
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
	type Item = Part;

	fn next(&mut self) -> Option<Self::Item> {
		self.tag_content_iter.next().map(|part_ref| match part_ref {
			PartRef::Text(text) => Part::Text(text.to_string()),
			PartRef::TagElemRef(tag_content) => Part::TagElem(TagElem {
				tag: tag_content.tag_name.to_string(),
				attrs: parse_attribute(tag_content.attrs_raw),
				content: tag_content.content.to_string(),
			}),
		})
	}
}

// region:    --- Tests

#[path = "tag_elem_iter_tests.rs"]
#[cfg(test)]
mod tests;

// endregion: --- Tests
