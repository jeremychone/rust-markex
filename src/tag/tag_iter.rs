//! Iterator to extract structured `TagElem`s including content and parsed attributes.
#![doc = include_str!("../../docs/rustdoc/tag/tag_iter.md")]

use super::{Part, TagOptions, TagRefIter};

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
	/// * `options` - Parser configuration, or `None` for default options.
	pub fn new(input: &'a str, tag_names: &[&'a str], options: impl Into<TagOptions>) -> Self {
		let options = options.into();
		let tag_names_vec: Vec<&'a str> = tag_names.to_vec();
		let tag_content_iter = TagRefIter::new(input, &tag_names_vec, options);

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
	/// * `options` - Parser configuration, or `None` for default options.
	pub fn new_single_tag(input: &'a str, tag_name: &'a str, options: impl Into<TagOptions>) -> Self {
		Self::new(input, &[tag_name], options)
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
