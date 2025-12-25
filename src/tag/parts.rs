use serde::Serialize;

use crate::tag::TagElem;

/// Represents a part of parsed content, either plain text or a tag element.
#[derive(Debug, Serialize, Clone, PartialEq)]
pub enum Part {
	/// Plain text content outside of any tag.
	Text(String),

	/// A tag element with its content.
	TagElem(TagElem),
}

/// Result of extracting data and parts from input.
#[derive(Debug, Serialize, Clone, PartialEq, Default)]
pub struct Parts {
	pub tag_names: Vec<String>,
	pub parts: Vec<Part>,
}

impl Parts {
	/// Returns references to all `TagElem` items in the parsed data.
	pub fn tag_elems(&self) -> Vec<&TagElem> {
		self.parts
			.iter()
			.filter_map(|part| match part {
				Part::TagElem(elem) => Some(elem),
				Part::Text(_) => None,
			})
			.collect()
	}

	/// Consumes the parsed data and returns all `TagElem` items.
	pub fn into_tag_elems(self) -> Vec<TagElem> {
		self.parts
			.into_iter()
			.filter_map(|part| match part {
				Part::TagElem(elem) => Some(elem),
				Part::Text(_) => None,
			})
			.collect()
	}

	/// Returns references to all text strings in the parsed data.
	pub fn texts(&self) -> Vec<&String> {
		self.parts
			.iter()
			.filter_map(|part| match part {
				Part::Text(text) => Some(text),
				Part::TagElem(_) => None,
			})
			.collect()
	}

	/// Consumes the parsed data and returns all text strings.
	pub fn into_texts(self) -> Vec<String> {
		self.parts
			.into_iter()
			.filter_map(|part| match part {
				Part::Text(text) => Some(text),
				Part::TagElem(_) => None,
			})
			.collect()
	}

	/// Consumes the parsed data and returns the tag elements along with all text concatenated into a single string.
	pub fn into_with_extrude_content(self) -> (Vec<TagElem>, String) {
		let mut tag_elems = Vec::new();
		let mut text_content = String::new();

		for part in self.parts {
			match part {
				Part::TagElem(elem) => tag_elems.push(elem),
				Part::Text(text) => text_content.push_str(&text),
			}
		}

		(tag_elems, text_content)
	}
}
