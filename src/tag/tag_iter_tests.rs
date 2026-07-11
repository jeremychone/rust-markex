//! Tests for the TagElemIter.

use crate::tag::{FENCE_BRACKETS, Part, TagElem, TagIter};
use std::collections::HashMap;

type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

#[test]
fn test_support_tag_elem_iter_simple() -> Result<()> {
	// -- Setup & Fixtures
	let text = "Some text <DATA>content1</DATA> more text <DATA>content2</DATA> final.";
	let tag_name = "DATA";

	// -- Exec
	let parts: Vec<Part> = TagIter::new_single_tag(text, tag_name, true).collect();

	// -- Check
	assert_eq!(parts.len(), 5);
	assert_eq!(parts[0], Part::Text("Some text ".to_string()));
	assert_eq!(
		parts[1],
		Part::TagElem(TagElem {
			tag: "DATA".to_string(),
			attrs: None,
			content: "content1".to_string()
		})
	);
	assert_eq!(parts[2], Part::Text(" more text ".to_string()));
	assert_eq!(
		parts[3],
		Part::TagElem(TagElem {
			tag: "DATA".to_string(),
			attrs: None,
			content: "content2".to_string()
		})
	);
	assert_eq!(parts[4], Part::Text(" final.".to_string()));

	Ok(())
}

#[test]
fn test_support_tag_elem_iter_bracket_fence_self_closing_spacing() -> Result<()> {
	// -- Setup & Fixtures
	let cases = [
		(r#"[[[DELETE path="canonical.txt"/]]]"#, "canonical.txt"),
		(r#"[[[DELETE path="alternate.txt" /]]"#, "alternate.txt"),
	];

	// -- Exec & Check
	for (text, expected_path) in cases {
		let parts: Vec<Part> = TagIter::new_with_fence(text, &["DELETE"], false, FENCE_BRACKETS).collect();
		let tag_elem = match parts.first() {
			Some(Part::TagElem(tag_elem)) => tag_elem,
			_ => return Err("should extract a self-closing DELETE element".into()),
		};

		assert_eq!(tag_elem.content, "");
		assert_eq!(
			tag_elem
				.attrs
				.as_ref()
				.and_then(|attrs| attrs.get("path"))
				.ok_or("should extract the DELETE path attribute")?,
			expected_path
		);
	}

	Ok(())
}

#[test]
fn test_support_tag_elem_iter_with_attrs() -> Result<()> {
	// -- Setup & Fixtures
	let text = r#"Some <DATA path="a/b.txt" flag attr=123 message='hello world'>value</DATA>"#;
	let tag_name = "DATA";

	// -- Exec
	let parts: Vec<Part> = TagIter::new_single_tag(text, tag_name, true).collect();

	// -- Check
	assert_eq!(parts.len(), 2);
	assert_eq!(parts[0], Part::Text("Some ".to_string()));

	let mut expected_attrs = HashMap::new();
	expected_attrs.insert("path".to_string(), "a/b.txt".to_string());
	expected_attrs.insert("flag".to_string(), "".to_string());
	expected_attrs.insert("attr".to_string(), "123".to_string());
	expected_attrs.insert("message".to_string(), "hello world".to_string());

	assert_eq!(
		parts[1],
		Part::TagElem(TagElem {
			tag: "DATA".to_string(),
			attrs: Some(expected_attrs.clone()),
			content: "value".to_string()
		})
	);

	Ok(())
}

#[test]
fn test_support_tag_elem_iter_no_tags() -> Result<()> {
	// -- Setup & Fixtures
	let text = "Some text without tags.";
	let tag_name = "DATA";

	// -- Exec
	let parts: Vec<Part> = TagIter::new_single_tag(text, tag_name, true).collect();

	// -- Check
	assert_eq!(parts.len(), 1);
	assert_eq!(parts[0], Part::Text(text.to_string()));

	Ok(())
}

#[test]
fn test_support_tag_elem_iter_multiple_tag_names() -> Result<()> {
	// -- Setup & Fixtures
	let text = "Alpha <ONE>first</ONE> Beta <TWO attr=ok>second</TWO> Gamma";
	let tag_names = ["ONE", "TWO"];

	// -- Exec
	let parts: Vec<Part> = TagIter::new(text, &tag_names, true).collect();

	// -- Check
	assert_eq!(parts.len(), 5);
	assert_eq!(parts[0], Part::Text("Alpha ".to_string()));

	assert_eq!(
		parts[1],
		Part::TagElem(TagElem {
			tag: "ONE".to_string(),
			attrs: None,
			content: "first".to_string()
		})
	);

	assert_eq!(parts[2], Part::Text(" Beta ".to_string()));

	let mut expected_attrs = HashMap::new();
	expected_attrs.insert("attr".to_string(), "ok".to_string());

	assert_eq!(
		parts[3],
		Part::TagElem(TagElem {
			tag: "TWO".to_string(),
			attrs: Some(expected_attrs),
			content: "second".to_string()
		})
	);

	assert_eq!(parts[4], Part::Text(" Gamma".to_string()));

	Ok(())
}

// region:    --- Self-closing tag tests

#[test]
fn test_support_tag_elem_iter_self_closing_simple() -> Result<()> {
	// -- Setup & Fixtures
	let text = "<DATA/>";
	let tag_name = "DATA";

	// -- Exec
	let parts: Vec<Part> = TagIter::new_single_tag(text, tag_name, true).collect();

	// -- Check
	assert_eq!(parts.len(), 1);
	assert_eq!(
		parts[0],
		Part::TagElem(TagElem {
			tag: "DATA".to_string(),
			attrs: None,
			content: "".to_string(),
		})
	);

	Ok(())
}

#[test]
fn test_support_tag_elem_iter_self_closing_with_attrs() -> Result<()> {
	// -- Setup & Fixtures
	let text = r#"<FILE path="a/b.txt" id=123 />"#;
	let tag_name = "FILE";

	// -- Exec
	let parts: Vec<Part> = TagIter::new_single_tag(text, tag_name, true).collect();

	// -- Check
	assert_eq!(parts.len(), 1);

	let mut expected_attrs = HashMap::new();
	expected_attrs.insert("path".to_string(), "a/b.txt".to_string());
	expected_attrs.insert("id".to_string(), "123".to_string());

	assert_eq!(
		parts[0],
		Part::TagElem(TagElem {
			tag: "FILE".to_string(),
			attrs: Some(expected_attrs),
			content: "".to_string(),
		})
	);

	Ok(())
}

#[test]
fn test_support_tag_elem_iter_self_closing_mixed() -> Result<()> {
	// -- Setup & Fixtures
	let text = "alpha <X/> beta <Y>content</Y> gamma";
	let tag_names = ["X", "Y"];

	// -- Exec
	let parts: Vec<Part> = TagIter::new(text, &tag_names, true).collect();

	// -- Check
	assert_eq!(parts.len(), 5);
	assert_eq!(parts[0], Part::Text("alpha ".to_string()));
	assert!(matches!(parts[1], Part::TagElem(_)));
	assert_eq!(parts[2], Part::Text(" beta ".to_string()));
	assert!(matches!(parts[3], Part::TagElem(_)));
	assert_eq!(parts[4], Part::Text(" gamma".to_string()));

	Ok(())
}

// endregion: --- Self-closing tag tests

#[test]
fn test_support_tag_elem_iter_bracket_fence_alternate_delimiters() -> Result<()> {
	// -- Setup & Fixtures
	let cases = [
		(r#"[[[DATA]]]canonical[[[END_DATA]]]"#, "canonical"),
		(r#"[[[DATA]]short-open[[[END_DATA]]]"#, "short-open"),
		(r#"[[[DATA]]]short-close[[[END_DATA]]"#, "short-close"),
		(r#"[[[DATA]]fully-short[[[END_DATA]]"#, "fully-short"),
	];

	// -- Exec & Check
	for (text, expected_content) in cases {
		let parts: Vec<Part> = TagIter::new_with_fence(text, &["DATA"], false, FENCE_BRACKETS).collect();

		assert_eq!(
			parts,
			vec![Part::TagElem(TagElem {
				tag: "DATA".to_string(),
				attrs: None,
				content: expected_content.to_string(),
			})]
		);
	}

	Ok(())
}
