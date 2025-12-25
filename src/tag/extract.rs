//! Parser module for extracting tag elements and text fragments from input.

use crate::tag::{Parts, TagIter};

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
	let iter = TagIter::new(input, tag_names, capture_text);
	let parts = iter.collect();

	Parts {
		tag_names: tag_names.iter().map(|s| s.to_string()).collect(),
		parts,
	}
}

// region:    --- Tests

#[path = "extract_tests.rs"]
#[cfg(test)]
mod tests;

// endregion: --- Tests
