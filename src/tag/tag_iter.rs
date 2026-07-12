//! Iterator to extract structured `TagElem`s including content and parsed attributes.
#![doc = include_str!("../../docs/rustdoc/tag/tag_iter.md")]

use super::{Part, TagFence, TagOptions, TagRefIter};

/// Iterator that yields owned `Part` instances (`Text` or `TagElem`), found within a text
/// based on specific tag names.
/// It consumes the referenced elements from `TagRefIter` and converts them to owned types.
pub struct TagIter<'a> {
	tag_content_iter: TagRefIter<'a>,
}

impl<'a> TagIter<'a> {
	/// Creates a new `TagIter`.
	///
	/// # Arguments
	///
	/// * `input` - The string slice to search within.
	/// * `tag_names` - A slice of tag names to search for (e.g., &["FILE", "DATA"]).
	/// * `capture_text` - If true, includes `Part::Text` fragments in the result.
	pub fn new(input: &'a str, tag_names: &[&'a str], capture_text: bool) -> Self {
		Self::new_with_options(input, tag_names, capture_text, TagOptions::default())
	}

	/// Creates a new `TagIter` using the provided tag fence.
	pub fn new_with_fence(input: &'a str, tag_names: &[&'a str], capture_text: bool, fence: TagFence) -> Self {
		Self::new_with_options(input, tag_names, capture_text, TagOptions::default().with_fence(fence))
	}

	/// Creates a new `TagIter` using the provided options.
	pub fn new_with_options(input: &'a str, tag_names: &[&'a str], capture_text: bool, options: TagOptions) -> Self {
		let tag_names_vec: Vec<&'a str> = tag_names.to_vec();
		let tag_content_iter = TagRefIter::new_with_options(input, &tag_names_vec, capture_text, options);

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
	/// * `capture_text` - If true, includes `Part::Text` fragments in the result.
	pub fn new_single_tag(input: &'a str, tag_name: &'a str, capture_text: bool) -> Self {
		Self::new(input, &[tag_name], capture_text)
	}
}

impl Iterator for TagIter<'_> {
	type Item = Part;

	fn next(&mut self) -> Option<Self::Item> {
		self.tag_content_iter.next().map(Part::from)
	}
}

// region:    --- Tests

#[path = "tag_iter_tests.rs"]
#[cfg(test)]
mod tests;

// endregion: --- Tests
