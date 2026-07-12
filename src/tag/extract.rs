//! Parser module for extracting tag elements and text fragments from input.

use crate::tag::{Parts, PartsRef, TagIter, TagOptions, TagRefIter};

/// Parses the input string for the specified tag names.
///
/// # Arguments
///
/// * `input` - The string slice to parse.
/// * `tag_names` - A slice of tag names to search for (e.g., &["FILE", "DATA"]).
/// * `options` - The parser configuration, including text-capture behavior.
///
/// # Returns
///
/// A `Parts` containing the extracted parts.
pub fn extract(input: &str, tag_names: &[&str], options: impl Into<TagOptions>) -> Parts {
	let options = options.into();
	let iter = TagIter::new(input, tag_names, options);
	let parts = iter.collect();

	Parts { parts }
}

/// Parses the input string for the specified tag names and returns references.
pub fn extract_refs<'a>(input: &'a str, tag_names: &[&str], options: impl Into<TagOptions>) -> PartsRef<'a> {
	let options = options.into();
	let iter = TagRefIter::new(input, tag_names, options);
	let parts = iter.collect();

	PartsRef { parts }
}

// region:    --- Tests

#[path = "extract_tests.rs"]
#[cfg(test)]
mod tests;

// endregion: --- Tests
