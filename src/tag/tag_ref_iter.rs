//! Iterator for extracting marked content sections like <TAG>...</TAG> from text.
#![doc = include_str!("../../docs/rustdoc/tag/tag_ref_iter.md")]

// region:    --- Types

use crate::tag::{TagElemRef, TagFence, FENCE_XML};
use crate::tag::support::parse_attrs_ref;

/// Represents a part of parsed content as a reference, either plain text or a tag element reference.
#[derive(Debug, PartialEq)]
pub enum PartRef<'a> {
	/// Plain text content outside of any tag.
	Text(&'a str),

	/// A tag element reference with its content.
	TagElemRef(TagElemRef<'a>),
}

/// Precomputed tag patterns derived from the tag name for efficient searching.
pub struct TagPattern {
	/// The original tag name (e.g., "FILE").
	pub name: String,
	/// The opening tag prefix (e.g., "<FILE"). Used to find the start of the tag.
	pub start_tag_prefix: String,
	/// The closing tag structure (e.g., "</FILE>"). Used to find the end of the element.
	pub end_tags: Vec<String>,
	/// The delimiters that end opening and closing tags.
	pub close_delims: Vec<&'static str>,
	/// The prefix between an opening delimiter and a closing tag name.
	pub closing_tag_prefix: String,
}

impl TagPattern {
	pub fn new(tag_name: &str, fence: TagFence) -> Self {
		let mut close_delims = vec![fence.close_delim];
		if let Some(close_delim_alts) = fence.close_delim_alts {
			close_delims.extend_from_slice(close_delim_alts);
		}
		let end_tags = close_delims
			.iter()
			.map(|close_delim| {
				format!(
					"{}{}{tag_name}{close_delim}",
					fence.open_delim, fence.closing_tag_prefix
				)
			})
			.collect();

		TagPattern {
			name: tag_name.to_string(),
			start_tag_prefix: format!("{}{tag_name}", fence.open_delim),
			end_tags,
			close_delims,
			closing_tag_prefix: fence.closing_tag_prefix.to_string(),
		}
	}
}

fn find_next_match<'a>(
	input: &str,
	patterns: impl IntoIterator<Item = &'a str>,
) -> Option<(usize, usize)> {
	let mut selected = None;

	for pattern in patterns {
		if let Some(start_idx) = input.find(pattern) {
			selected = match selected {
				None => Some((start_idx, pattern.len())),
				Some((existing_idx, existing_len)) => {
					if start_idx < existing_idx || (start_idx == existing_idx && pattern.len() > existing_len) {
						Some((start_idx, pattern.len()))
					} else {
						Some((existing_idx, existing_len))
					}
				}
			};
		}
	}

	selected
}

// endregion: --- Types

/// An iterator that finds and extracts `PartRef` sections from a string slice.
///
/// It searches for pairs of opening `<TAG_NAME...>` and closing `</TAG_NAME>` tags
/// for one or more configured tag names, yielding both text fragments and tag elements.
pub struct TagRefIter<'a> {
	input: &'a str,
	current_pos: usize,
	last_processed_idx: usize,
	tag_patterns: Vec<TagPattern>,
	pending_tag: Option<TagElemRef<'a>>,
	finished: bool,
	capture_text: bool,
}

impl<'a> TagRefIter<'a> {
	/// Creates a new `TagRefIter` for the given input string and tag names.
	///
	/// # Arguments
	///
	/// * `input` - The string slice to search within.
	/// * `tag_names` - The names of the tags to search for (e.g., &["FILE", "DATA"]).
	/// * `capture_text` - If true, includes `PartRef::Text` fragments in the result.
	pub fn new(input: &'a str, tag_names: &[&str], capture_text: bool) -> Self {
		Self::new_with_fence(input, tag_names, capture_text, FENCE_XML)
	}

	/// Creates a new `TagRefIter` using the provided tag fence.
	pub fn new_with_fence(input: &'a str, tag_names: &[&str], capture_text: bool, fence: TagFence) -> Self {
		let tag_infos = tag_names.iter().map(|&name| TagPattern::new(name, fence)).collect();
		TagRefIter {
			input,
			current_pos: 0,
			last_processed_idx: 0,
			tag_patterns: tag_infos,
			pending_tag: None,
			finished: false,
			capture_text,
		}
	}

	/// Internal method to find the next tag element.
	fn find_next_tag(&mut self) -> Option<TagElemRef<'a>> {
		while self.current_pos < self.input.len() {
			// --- Find the start tag prefix ---
			let remaining_input = &self.input[self.current_pos..];
			let mut selected: Option<(usize, &TagPattern)> = None;

			for tag_info in &self.tag_patterns {
				if let Some(offset) = remaining_input.find(&tag_info.start_tag_prefix) {
					let start_idx = self.current_pos + offset;

					selected = match selected {
						None => Some((start_idx, tag_info)),
						Some((existing_idx, existing_tag)) => {
							if start_idx < existing_idx
								|| (start_idx == existing_idx && tag_info.name.len() > existing_tag.name.len())
							{
								Some((start_idx, tag_info))
							} else {
								Some((existing_idx, existing_tag))
							}
						}
					};
				}
			}

			let (start_idx, tag_info) = selected?;

			let after_prefix_idx = start_idx + tag_info.start_tag_prefix.len();

			// --- Validate character after prefix (must be the closing delimiter, closing prefix, or whitespace) ---
			let remaining_after_prefix = &self.input[after_prefix_idx..];
			let valid_after_prefix = tag_info
				.close_delims
				.iter()
				.any(|close_delim| remaining_after_prefix.starts_with(close_delim))
				|| remaining_after_prefix.starts_with(&tag_info.closing_tag_prefix)
				|| matches!(
					self.input.as_bytes().get(after_prefix_idx),
					Some(b' ') | Some(b'\n') | Some(b'\t') | Some(b'\r')
				);
			if !valid_after_prefix {
				// It's a different tag (e.g., <TAG_NAMEXXX). Advance past the '<' and continue searching.
				self.current_pos = start_idx + 1;
				continue;
			}

			// --- Find the end of the opening tag ---
			let remaining_from_start = &self.input[start_idx..];
			let (open_tag_end_offset, close_delim_len) =
				match find_next_match(remaining_from_start, tag_info.close_delims.iter().copied()) {
					Some(match_info) => match_info,
				None => {
					// Malformed open tag (no '>'). Stop searching. Consider advancing past '<'?
					// For simplicity, we stop here. A more robust parser might skip.
					return None;
				}
			};
			let open_tag_close_start_idx = start_idx + open_tag_end_offset;
			let open_tag_end_idx = open_tag_close_start_idx + close_delim_len - 1;

			let tag_name_len = tag_info.name.len();
			let tag_name_start_idx = after_prefix_idx - tag_name_len;
			let tag_name = &self.input[tag_name_start_idx..after_prefix_idx];

			// --- Check for self-closing tag ---
			let opening_tag_body = &self.input[after_prefix_idx..open_tag_close_start_idx];
			let self_closing = opening_tag_body.ends_with(&tag_info.closing_tag_prefix);

			// --- Extract Parameters (exclude self-closing slash) ---
			let attrs_section = if self_closing {
				&opening_tag_body[..opening_tag_body.len() - tag_info.closing_tag_prefix.len()]
			} else {
				opening_tag_body
			};
			let attrs = parse_attrs_ref(Some(attrs_section));

			if self_closing {
				// Self-closing: no content, no closing tag search
				let end_idx = open_tag_end_idx;
				self.current_pos = end_idx + 1;
				return Some(TagElemRef {
					tag_name,
					attrs,
					content: "",
					start_idx,
					end_idx,
				});
			}

			// --- Find the closing tag ---
			let search_after_open_tag_idx = open_tag_close_start_idx + close_delim_len;
			if search_after_open_tag_idx >= self.input.len() {
				// Reached end of input before finding closing tag
				return None;
			}

			let remaining_after_open = &self.input[search_after_open_tag_idx..];
			let (close_tag_start_offset, close_tag_len) =
				find_next_match(remaining_after_open, tag_info.end_tags.iter().map(String::as_str))?;
			let close_tag_start_idx = search_after_open_tag_idx + close_tag_start_offset;
			// Corrected end_idx calculation: it's the index of the '>' of the closing tag
			// The end index should be the index of the last character of the closing tag '>'
			let end_idx = close_tag_start_idx + close_tag_len - 1;

			// --- Extract Content ---
			let content = &self.input[open_tag_end_idx + 1..close_tag_start_idx];

			// --- Update position for next search ---
			// The next search should start right after the closing tag
			self.current_pos = end_idx + 1;

			// --- Return the found item ---
			return Some(TagElemRef {
				tag_name,
				attrs,
				content,
				start_idx,
				end_idx,
			});
		}

		None
	}
}

impl<'a> Iterator for TagRefIter<'a> {
	type Item = PartRef<'a>;

	fn next(&mut self) -> Option<Self::Item> {
		// If we have a pending tag, yield it
		if let Some(tag) = self.pending_tag.take() {
			self.last_processed_idx = tag.end_idx + 1;
			return Some(PartRef::TagElemRef(tag));
		}

		// If already finished, return None
		if self.finished {
			return None;
		}

		// Try to find the next tag
		if let Some(tag) = self.find_next_tag() {
			// Check if there's text before this tag
			if self.capture_text && tag.start_idx > self.last_processed_idx {
				let text = &self.input[self.last_processed_idx..tag.start_idx];
				self.pending_tag = Some(tag);
				return Some(PartRef::Text(text));
			} else {
				// No text before, yield the tag directly
				self.last_processed_idx = tag.end_idx + 1;
				return Some(PartRef::TagElemRef(tag));
			}
		}

		// No more tags found, check if there's remaining text
		self.finished = true;
		if self.capture_text && self.last_processed_idx < self.input.len() {
			let text = &self.input[self.last_processed_idx..];
			self.last_processed_idx = self.input.len();
			return Some(PartRef::Text(text));
		}

		None
	}
}

// region:    --- Tests

#[path = "tag_ref_iter_tests.rs"]
#[cfg(test)]
mod tests;

// endregion: --- Tests
