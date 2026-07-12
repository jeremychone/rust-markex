//! Parser module for extracting tag elements and text fragments from input.

use crate::tag::{Parts, PartsRef, TagFence, TagIter, TagOptions, TagRefIter};

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
pub fn extract(input: &str, tag_names: &[&str], capture_text: bool) -> Parts {
	extract_with_options(input, tag_names, capture_text, TagOptions::default())
}

/// Parses the input string for the specified tag names using the provided fence.
pub fn extract_with_fence(input: &str, tag_names: &[&str], capture_text: bool, fence: TagFence) -> Parts {
	extract_with_options(input, tag_names, capture_text, TagOptions::default().with_fence(fence))
}

/// Parses the input string for the specified tag names using the provided options.
pub fn extract_with_options(input: &str, tag_names: &[&str], capture_text: bool, options: TagOptions) -> Parts {
	let iter = TagIter::new_with_options(input, tag_names, capture_text, options);
	let parts = iter.collect();

	Parts { parts }
}

/// Parses the input string for the specified tag names and returns references.
pub fn extract_refs<'a>(input: &'a str, tag_names: &[&str], capture_text: bool) -> PartsRef<'a> {
	extract_refs_with_options(input, tag_names, capture_text, TagOptions::default())
}

/// Parses the input string for the specified tag names using the provided fence and returns references.
pub fn extract_refs_with_fence<'a>(
	input: &'a str,
	tag_names: &[&str],
	capture_text: bool,
	fence: TagFence,
) -> PartsRef<'a> {
	extract_refs_with_options(input, tag_names, capture_text, TagOptions::default().with_fence(fence))
}

/// Parses the input string for the specified tag names using the provided options and returns references.
pub fn extract_refs_with_options<'a>(
	input: &'a str,
	tag_names: &[&str],
	capture_text: bool,
	options: TagOptions,
) -> PartsRef<'a> {
	let iter = TagRefIter::new_with_options(input, tag_names, capture_text, options);
	let parts = iter.collect();

	PartsRef { parts }
}

// region:    --- Tests

#[path = "extract_tests.rs"]
#[cfg(test)]
mod tests;

// endregion: --- Tests
