//! Definition of the extracted element structure.

use serde::Serialize;
use std::collections::HashMap;

// region:    --- TagElem

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

impl From<crate::tag::TagElemRef<'_>> for TagElem {
	fn from(tag_ref: crate::tag::TagElemRef<'_>) -> Self {
		TagElem {
			tag: tag_ref.tag_name.to_string(),
			attrs: tag_ref
				.attrs
				.map(|attrs| attrs.into_iter().map(|(k, v)| (k.to_string(), v.to_string())).collect()),
			content: tag_ref.content.to_string(),
		}
	}
}

// endregion: --- TagElem
