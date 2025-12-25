//! Tests for the parser module.

use super::extract;
use crate::tag::{Part, TagElem};
use std::collections::HashMap;

type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

#[test]
fn test_tag_parser_simple_with_text() -> Result<()> {
	// -- Setup & Fixtures
	let input = "Before <DATA>content</DATA> After";
	let tag_names = ["DATA"];

	// -- Exec
	let result = extract(input, &tag_names, true);

	// -- Check
	assert_eq!(result.tag_names(), vec!["DATA"]);
	assert_eq!(result.parts().len(), 3);
	assert_eq!(result.parts()[0], Part::Text("Before ".to_string()));
	assert_eq!(
		result.parts()[1],
		Part::TagElem(TagElem {
			tag: "DATA".to_string(),
			attrs: None,
			content: "content".to_string(),
		})
	);
	assert_eq!(result.parts()[2], Part::Text(" After".to_string()));

	Ok(())
}

#[test]
fn test_tag_parser_simple_without_text() -> Result<()> {
	// -- Setup & Fixtures
	let input = "Before <DATA>content</DATA> After";
	let tag_names = ["DATA"];

	// -- Exec
	let result = extract(input, &tag_names, false);

	// -- Check
	assert_eq!(result.parts().len(), 1);
	assert_eq!(
		result.parts()[0],
		Part::TagElem(TagElem {
			tag: "DATA".to_string(),
			attrs: None,
			content: "content".to_string(),
		})
	);

	Ok(())
}

#[test]
fn test_tag_parser_multiple_tags_with_attrs() -> Result<()> {
	// -- Setup & Fixtures
	let input = r#"Start <FILE path="a.txt">file content</FILE> middle <DATA id=123>data content</DATA> end"#;
	let tag_names = ["FILE", "DATA"];

	// -- Exec
	let result = extract(input, &tag_names, true);

	// -- Check
	assert_eq!(result.parts().len(), 5);

	assert_eq!(result.parts()[0], Part::Text("Start ".to_string()));

	let mut file_attrs = HashMap::new();
	file_attrs.insert("path".to_string(), "a.txt".to_string());
	assert_eq!(
		result.parts()[1],
		Part::TagElem(TagElem {
			tag: "FILE".to_string(),
			attrs: Some(file_attrs),
			content: "file content".to_string(),
		})
	);

	assert_eq!(result.parts()[2], Part::Text(" middle ".to_string()));

	let mut data_attrs = HashMap::new();
	data_attrs.insert("id".to_string(), "123".to_string());
	assert_eq!(
		result.parts()[3],
		Part::TagElem(TagElem {
			tag: "DATA".to_string(),
			attrs: Some(data_attrs),
			content: "data content".to_string(),
		})
	);

	assert_eq!(result.parts()[4], Part::Text(" end".to_string()));

	Ok(())
}

#[test]
fn test_tag_parser_no_tags() -> Result<()> {
	// -- Setup & Fixtures
	let input = "Just plain text without any tags.";
	let tag_names = ["DATA"];

	// -- Exec
	let result_with_text = extract(input, &tag_names, true);
	let result_without_text = extract(input, &tag_names, false);

	// -- Check
	assert_eq!(result_with_text.parts().len(), 1);
	assert_eq!(
		result_with_text.parts()[0],
		Part::Text("Just plain text without any tags.".to_string())
	);

	assert!(result_without_text.parts().is_empty());

	Ok(())
}

#[test]
fn test_tag_parser_empty_input() -> Result<()> {
	// -- Setup & Fixtures
	let input = "";
	let tag_names = ["DATA"];

	// -- Exec
	let result = extract(input, &tag_names, true);

	// -- Check
	assert!(result.parts().is_empty());

	Ok(())
}

#[test]
fn test_tag_parser_only_tag() -> Result<()> {
	// -- Setup & Fixtures
	let input = "<DATA>content</DATA>";
	let tag_names = ["DATA"];

	// -- Exec
	let result = extract(input, &tag_names, true);

	// -- Check
	assert_eq!(result.parts().len(), 1);
	assert_eq!(
		result.parts()[0],
		Part::TagElem(TagElem {
			tag: "DATA".to_string(),
			attrs: None,
			content: "content".to_string(),
		})
	);

	Ok(())
}

#[test]
fn test_tag_parser_adjacent_tags() -> Result<()> {
	// -- Setup & Fixtures
	let input = "<A>first</A><B>second</B>";
	let tag_names = ["A", "B"];

	// -- Exec
	let result = extract(input, &tag_names, true);

	// -- Check
	assert_eq!(result.parts().len(), 2);
	assert_eq!(
		result.parts()[0],
		Part::TagElem(TagElem {
			tag: "A".to_string(),
			attrs: None,
			content: "first".to_string(),
		})
	);
	assert_eq!(
		result.parts()[1],
		Part::TagElem(TagElem {
			tag: "B".to_string(),
			attrs: None,
			content: "second".to_string(),
		})
	);

	Ok(())
}

#[test]
fn test_tag_parser_tag_elems() -> Result<()> {
	// -- Setup & Fixtures
	let input = "Before <DATA>content1</DATA> middle <DATA>content2</DATA> After";
	let tag_names = ["DATA"];

	// -- Exec
	let result = extract(input, &tag_names, true);
	let tag_elems = result.tag_elems();

	// -- Check
	assert_eq!(tag_elems.len(), 2);
	assert_eq!(tag_elems[0].tag, "DATA");
	assert_eq!(tag_elems[0].content, "content1");
	assert_eq!(tag_elems[1].tag, "DATA");
	assert_eq!(tag_elems[1].content, "content2");

	Ok(())
}

#[test]
fn test_tag_parser_into_tag_elems() -> Result<()> {
	// -- Setup & Fixtures
	let input = "Before <DATA>content1</DATA> middle <DATA>content2</DATA> After";
	let tag_names = ["DATA"];

	// -- Exec
	let result = extract(input, &tag_names, true);
	let tag_elems = result.into_tag_elems();

	// -- Check
	assert_eq!(tag_elems.len(), 2);
	assert_eq!(tag_elems[0].tag, "DATA");
	assert_eq!(tag_elems[0].content, "content1");
	assert_eq!(tag_elems[1].tag, "DATA");
	assert_eq!(tag_elems[1].content, "content2");

	Ok(())
}

#[test]
fn test_tag_parser_texts() -> Result<()> {
	// -- Setup & Fixtures
	let input = "Before <DATA>content</DATA> After";
	let tag_names = ["DATA"];

	// -- Exec
	let result = extract(input, &tag_names, true);
	let texts = result.texts();

	// -- Check
	assert_eq!(texts.len(), 2);
	assert_eq!(texts[0], "Before ");
	assert_eq!(texts[1], " After");

	Ok(())
}

#[test]
fn test_tag_parser_into_texts() -> Result<()> {
	// -- Setup & Fixtures
	let input = "Before <DATA>content</DATA> After";
	let tag_names = ["DATA"];

	// -- Exec
	let result = extract(input, &tag_names, true);
	let texts = result.into_texts();

	// -- Check
	assert_eq!(texts.len(), 2);
	assert_eq!(texts[0], "Before ");
	assert_eq!(texts[1], " After");

	Ok(())
}

#[test]
fn test_tag_parser_into_with_extrude_content() -> Result<()> {
	// -- Setup & Fixtures
	let input = "Before <DATA>content1</DATA> middle <DATA>content2</DATA> After";
	let tag_names = ["DATA"];

	// -- Exec
	let result = extract(input, &tag_names, true);
	let (tag_elems, text_content) = result.into_with_extrude_content();

	// -- Check
	assert_eq!(tag_elems.len(), 2);
	assert_eq!(tag_elems[0].tag, "DATA");
	assert_eq!(tag_elems[0].content, "content1");
	assert_eq!(tag_elems[1].tag, "DATA");
	assert_eq!(tag_elems[1].content, "content2");
	assert_eq!(text_content, "Before  middle  After");

	Ok(())
}
