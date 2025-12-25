//! Tests for the TagElemIter.

use crate::tag::{Part, TagElem, TagIter};
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
