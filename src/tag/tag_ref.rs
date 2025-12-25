/// Represents a segment of text identified by start and end tags,
/// potentially including parameters in the start marker.
///
/// Lifetimes ensure that all string slices (`tag_name`, `attrs_raw`, `content`)
/// are valid references to the original input string slice provided
/// to the `TagElemRefIterator`.
#[derive(Debug, PartialEq)]
pub struct TagElemRef<'a> {
	/// The name of the tag (e.g., "SOME_MARKER").
	pub tag_name: &'a str,

	/// Optional attributes string found within the opening tag.
	/// (e.g., `file_path="some/path/file.txt" other="123"`)
	/// This includes the raw string between the tag name and the closing '>'.
	pub attrs_raw: Option<&'a str>,

	/// The content string between the opening and closing tags.
	pub content: &'a str,

	/// The byte index of the opening '<' of the start tag in the original string.
	pub start_idx: usize,

	/// The byte index of the closing '>' of the end tag in the original string.
	pub end_idx: usize,
}
