use std::collections::HashMap;

/// Represents a segment of text identified by start and end tags,
/// potentially including parameters in the start marker.
///
/// Lifetimes ensure that all string slices (`tag_name`, `attrs`, `content`)
/// are valid references to the original input string slice provided
/// to the `TagElemRefIterator`.
#[derive(Debug, PartialEq)]
pub struct TagElemRef<'a> {
	/// The name of the tag (e.g., "SOME_MARKER").
	pub tag_name: &'a str,

	/// Optional attributes map.
	pub attrs: Option<HashMap<&'a str, &'a str>>,

	/// The content string between the opening and closing tags.
	pub content: &'a str,

	/// The byte index of the opening '<' of the start tag in the original string.
	pub start_idx: usize,

	/// The byte index of the closing '>' of the end tag in the original string.
	pub end_idx: usize,
}
