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

impl<'a> From<crate::tag::PartRef<'a>> for Part {
	fn from(part_ref: crate::tag::PartRef<'a>) -> Self {
		match part_ref {
			crate::tag::PartRef::Text(text) => Part::Text(text.to_string()),
			crate::tag::PartRef::TagElemRef(tag_ref) => Part::TagElem(TagElem::from(tag_ref)),
		}
	}
}

/// Result of extracting data and parts from input.
#[derive(Debug, Serialize, Clone, PartialEq, Default)]
pub struct Parts {
	pub(crate) parts: Vec<Part>,
}

impl Parts {
	pub fn parts(&self) -> &Vec<Part> {
		&self.parts
	}

	pub fn into_parts(self) -> Vec<Part> {
		self.parts
	}

	/// Returns the unique tag names found in the parts.
	pub fn tag_names(&self) -> Vec<&str> {
		let mut names = Vec::new();
		for part in &self.parts {
			if let Part::TagElem(elem) = part {
				if !names.contains(&elem.tag.as_str()) {
					names.push(elem.tag.as_str());
				}
			}
		}
		names
	}

	pub fn iter(&self) -> std::slice::Iter<'_, Part> {
		self.parts.iter()
	}

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

impl IntoIterator for Parts {
	type Item = Part;
	type IntoIter = std::vec::IntoIter<Self::Item>;

	fn into_iter(self) -> Self::IntoIter {
		self.parts.into_iter()
	}
}

impl<'a> IntoIterator for &'a Parts {
	type Item = &'a Part;
	type IntoIter = std::slice::Iter<'a, Part>;

	fn into_iter(self) -> Self::IntoIter {
		self.parts.iter()
	}
}

impl From<Parts> for Vec<Part> {
	fn from(val: Parts) -> Self {
		val.parts
	}
}
