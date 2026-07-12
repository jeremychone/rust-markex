//! Tests for the parser module.

use super::extract;
use crate::tag::{FENCE_BRACKETS, Part, TagElem, TagFence, TagOptions};
use std::collections::HashMap;

type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

#[test]
fn test_tag_parser_simple_with_text() -> Result<()> {
	// -- Setup & Fixtures
	let input = "Before <DATA>content</DATA> After";
	let tag_names = ["DATA"];

	// -- Exec
	let result = extract(input, &tag_names, TagOptions::default().with_capture_text(true));

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
			auto_closed: false,
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
	let result = extract(input, &tag_names, None);

	// -- Check
	assert_eq!(result.parts().len(), 1);
	assert_eq!(
		result.parts()[0],
		Part::TagElem(TagElem {
			tag: "DATA".to_string(),
			attrs: None,
			content: "content".to_string(),
			auto_closed: false,
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
	let result = extract(input, &tag_names, TagOptions::default().with_capture_text(true));

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
			auto_closed: false,
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
			auto_closed: false,
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
	let result_with_text = extract(input, &tag_names, TagOptions::default().with_capture_text(true));
	let result_without_text = extract(input, &tag_names, None);

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
	let result = extract(input, &tag_names, TagOptions::default().with_capture_text(true));

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
	let result = extract(input, &tag_names, TagOptions::default().with_capture_text(true));

	// -- Check
	assert_eq!(result.parts().len(), 1);
	assert_eq!(
		result.parts()[0],
		Part::TagElem(TagElem {
			tag: "DATA".to_string(),
			attrs: None,
			content: "content".to_string(),
			auto_closed: false,
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
	let result = extract(input, &tag_names, TagOptions::default().with_capture_text(true));

	// -- Check
	assert_eq!(result.parts().len(), 2);
	assert_eq!(
		result.parts()[0],
		Part::TagElem(TagElem {
			tag: "A".to_string(),
			attrs: None,
			content: "first".to_string(),
			auto_closed: false,
		})
	);
	assert_eq!(
		result.parts()[1],
		Part::TagElem(TagElem {
			tag: "B".to_string(),
			attrs: None,
			content: "second".to_string(),
			auto_closed: false,
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
	let result = extract(input, &tag_names, TagOptions::default().with_capture_text(true));
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
	let result = extract(input, &tag_names, TagOptions::default().with_capture_text(true));
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
	let result = extract(input, &tag_names, TagOptions::default().with_capture_text(true));
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
	let result = extract(input, &tag_names, TagOptions::default().with_capture_text(true));
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
	let result = extract(input, &tag_names, TagOptions::default().with_capture_text(true));
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

#[test]
fn test_tag_parser_bracket3_fence() -> Result<()> {
	// -- Setup & Fixtures
	let input = r#"Before [[[FILE path="a.txt"]]]file content[[[/FILE]]] after [[[DELETE path="temp.txt" /]]] end"#;
	let tag_names = ["FILE", "DELETE"];

	// -- Exec
	let result = extract(
		input,
		&tag_names,
		TagOptions::default().with_capture_text(true).with_fence(FENCE_BRACKETS),
	);

	// -- Check
	assert_eq!(result.parts().len(), 5);
	assert_eq!(result.parts()[0], Part::Text("Before ".to_string()));

	let mut file_attrs = HashMap::new();
	file_attrs.insert("path".to_string(), "a.txt".to_string());
	assert_eq!(
		result.parts()[1],
		Part::TagElem(TagElem {
			tag: "FILE".to_string(),
			attrs: Some(file_attrs),
			content: "file content".to_string(),
			auto_closed: false,
		})
	);

	assert_eq!(result.parts()[2], Part::Text(" after ".to_string()));

	let mut delete_attrs = HashMap::new();
	delete_attrs.insert("path".to_string(), "temp.txt".to_string());
	assert_eq!(
		result.parts()[3],
		Part::TagElem(TagElem {
			tag: "DELETE".to_string(),
			attrs: Some(delete_attrs),
			content: "".to_string(),
			auto_closed: false,
		})
	);

	assert_eq!(result.parts()[4], Part::Text(" end".to_string()));

	Ok(())
}

#[test]
fn test_tag_parser_custom_fence() -> Result<()> {
	// -- Setup & Fixtures
	let fence = TagFence {
		name: "mustache",
		open_delim: "{{",
		close_delim: "}}",
		close_delim_alts: None,
		closing_tag_prefix: "/",
		self_closing_suffix: "/",
	};
	let input = "{{DATA key=value}}payload{{/DATA}}";
	let tag_names = ["DATA"];

	// -- Exec
	let result = extract(input, &tag_names, TagOptions::default().with_fence(fence));

	// -- Check
	let mut attrs = HashMap::new();
	attrs.insert("key".to_string(), "value".to_string());
	assert_eq!(
		result.parts(),
		&[Part::TagElem(TagElem {
			tag: "DATA".to_string(),
			attrs: Some(attrs),
			content: "payload".to_string(),
			auto_closed: false,
		})]
	);

	Ok(())
}

#[test]
fn test_tag_parser_bracket3_fence_with_alternate_delimiters() -> Result<()> {
	// -- Setup & Fixtures
	let cases = [
		(r#"[[[FILE]]]canonical[[[/FILE]]]"#, "canonical"),
		(r#"[[[FILE]]short-open[[[/FILE]]]"#, "short-open"),
		(r#"[[[FILE]]]short-close[[[/FILE]]"#, "short-close"),
		(r#"[[[FILE]]fully-short[[[/FILE]]"#, "fully-short"),
	];

	// -- Exec & Check
	for (input, expected_content) in cases {
		let result = extract(input, &["FILE"], TagOptions::default().with_fence(FENCE_BRACKETS));
		let tag_elems = result.tag_elems();
		let tag_elem = tag_elems.first().ok_or("should extract a FILE element")?;

		assert_eq!(tag_elem.content, expected_content);
	}

	let self_closing = extract(
		r#"[[[DELETE path="temp.txt" /]]]"#,
		&["DELETE"],
		TagOptions::default().with_fence(FENCE_BRACKETS),
	);
	let delete_elems = self_closing.tag_elems();
	let delete_elem = delete_elems.first().ok_or("should extract a self-closing DELETE element")?;

	assert_eq!(delete_elem.content, "");
	assert_eq!(
		delete_elem
			.attrs
			.as_ref()
			.and_then(|attrs| attrs.get("path"))
			.ok_or("should extract the DELETE path attribute")?,
		"temp.txt"
	);

	let compact_self_closing = extract(
		r#"[[[DELETE path="cache.txt"/]]]"#,
		&["DELETE"],
		TagOptions::default().with_fence(FENCE_BRACKETS),
	);
	let compact_delete_elems = compact_self_closing.tag_elems();
	let compact_delete_elem = compact_delete_elems
		.first()
		.ok_or("should extract a compact self-closing DELETE element")?;

	assert_eq!(compact_delete_elem.content, "");
	assert_eq!(
		compact_delete_elem
			.attrs
			.as_ref()
			.and_then(|attrs| attrs.get("path"))
			.ok_or("should extract the compact DELETE path attribute")?,
		"cache.txt"
	);

	Ok(())
}

#[test]
fn test_tag_parser_extract_with_default_and_fence_options() -> Result<()> {
	// -- Setup & Fixtures
	let xml_input = "Before <DATA>content</DATA> After";
	let bracket_input = r#"Before [[[DATA]]]content[[[/DATA]]] After"#;
	let tag_names = ["DATA"];
	let default_options = TagOptions::default();
	let bracket_options = TagOptions::default().with_fence(FENCE_BRACKETS);

	// -- Exec
	let default_result = extract(
		xml_input,
		&tag_names,
		default_options.with_capture_text(true),
	);
	let existing_default_result = extract(xml_input, &tag_names, TagOptions::default().with_capture_text(true));
	let option_fence_result = extract(
		bracket_input,
		&tag_names,
		bracket_options.with_capture_text(true),
	);
	let existing_fence_result = extract(
		bracket_input,
		&tag_names,
		TagOptions::default().with_capture_text(true).with_fence(FENCE_BRACKETS),
	);

	// -- Check
	assert_eq!(default_options.fence, None);
	assert_eq!(default_result.parts(), existing_default_result.parts());
	assert_eq!(option_fence_result.parts(), existing_fence_result.parts());

	Ok(())
}

#[test]
fn test_tag_extract_auto_close_with_options_and_normal_close_precedence() -> Result<()> {
	// -- Setup & Fixtures
	let auto_close_input = "<FILE>first <DATA>second</DATA>";
	let normally_closed_input = "<FILE>first</FILE><DATA>second</DATA>";
	let tag_names = ["FILE", "DATA"];
	let options = TagOptions::default().with_auto_close(true);

	// -- Exec
	let auto_close_result = extract(auto_close_input, &tag_names, options);
	let normally_closed_result = extract(normally_closed_input, &tag_names, options);

	// -- Check
	let auto_close_tags = auto_close_result.tag_elems();
	let auto_closed_file = auto_close_tags.first().ok_or("should extract an auto-closed FILE element")?;
	let data_tag = auto_close_tags.get(1).ok_or("should extract the following DATA element")?;

	assert_eq!(auto_closed_file.content, "first ");
	assert!(auto_closed_file.auto_closed);
	assert_eq!(data_tag.content, "second");
	assert!(!data_tag.auto_closed);

	let normally_closed_tags = normally_closed_result.tag_elems();
	assert_eq!(normally_closed_tags.len(), 2);
	assert!(!normally_closed_tags[0].auto_closed);
	assert!(!normally_closed_tags[1].auto_closed);

	Ok(())
}
