//! Definition of the extracted element structure.

use serde::Serialize;
use std::collections::HashMap;

/// Represents a part of parsed content, either plain text or a tag element.
#[derive(Debug, Serialize, Clone, PartialEq)]
pub enum Part {
	/// Plain text content outside of any tag.
	Text(String),

	/// A tag element with its content.
	TagElem(TagElem),
}

/// Represents a block defined by start and end tags, like `<TAG>content</TAG>`.
#[derive(Debug, Serialize, Clone, PartialEq, Default)]
pub struct TagElem {
	pub tag: String, // might want to set this a Arc<str>

	pub attrs: Option<HashMap<String, String>>,

	pub content: String,
}

impl TagElem {
	/// Creates a new `TagElem` with the specified name, optional attributes, and content.
	pub fn new(name: impl Into<String>, attrs: Option<HashMap<String, String>>, content: impl Into<String>) -> Self {
		TagElem {
			tag: name.into(),
			attrs,
			content: content.into(),
		}
	}
}
