//! Tests for the TagContentIterator.

use super::{PartRef, TagElemRef, TagRefIter};
use std::collections::HashMap;
use std::error::Error;
// For tests, using a simple Result alias is often sufficient.
type Result<T> = core::result::Result<T, Box<dyn Error>>;

// -- Helper to extract TagElemRef from PartRef

fn extract_tag_elem_refs<'a>(parts: Vec<PartRef<'a>>) -> Vec<TagElemRef<'a>> {
	parts
		.into_iter()
		.filter_map(|p| match p {
			PartRef::TagElemRef(tag) => Some(tag),
			PartRef::Text(_) => None,
		})
		.collect()
}

#[test]
fn test_support_tag_content_iter_simple() -> Result<()> {
	// -- Setup & Fixtures
	let text = "Some text <DATA>content</DATA> more text.";
	let tag_name = "DATA";

	// -- Exec
	let parts: Vec<PartRef> = TagRefIter::new(text, &[tag_name], true).collect();
	let tags = extract_tag_elem_refs(parts);

	// -- Check
	assert_eq!(tags.len(), 1);

	assert_eq!(
		tags[0],
		TagElemRef {
			tag_name: "DATA",
			attrs: None,
			content: "content",
			start_idx: 10,
			end_idx: 29,
		}
	);

	Ok(())
}

#[test]
fn test_support_tag_content_iter_attrs() -> Result<()> {
	// -- Setup & Fixtures
	let text = r#"Before <FILE path="a/b.txt" id=123>File Content</FILE> After"#;
	let tag_name = "FILE";

	// -- Exec
	let parts: Vec<PartRef> = TagRefIter::new(text, &[tag_name], true).collect();
	let tags = extract_tag_elem_refs(parts);

	// -- Check
	assert_eq!(tags.len(), 1);

	let mut expected_attrs = HashMap::new();
	expected_attrs.insert("path", "a/b.txt");
	expected_attrs.insert("id", "123");

	assert_eq!(
		tags[0],
		TagElemRef {
			tag_name: "FILE",
			attrs: Some(expected_attrs),
			content: "File Content",
			start_idx: 7,
			end_idx: 53,
		}
	);

	Ok(())
}

#[test]
fn test_support_tag_content_iter_attrs_with_newline() -> Result<()> {
	// -- Setup & Fixtures
	let text = "Before <FILE \npath=\"a/b.txt\"\n id=123>File Content</FILE> After";
	let tag_name = "FILE";

	// -- Exec
	let parts: Vec<PartRef> = TagRefIter::new(text, &[tag_name], true).collect();
	let tags = extract_tag_elem_refs(parts);

	// -- Check
	assert_eq!(tags.len(), 1);

	let mut expected_attrs = HashMap::new();
	expected_attrs.insert("path", "a/b.txt");
	expected_attrs.insert("id", "123");

	assert_eq!(
		tags[0],
		TagElemRef {
			tag_name: "FILE",
			attrs: Some(expected_attrs),
			content: "File Content",
			start_idx: 7,
			end_idx: 55,
		}
	);

	Ok(())
}

#[test]
fn test_support_tag_content_iter_multiple() -> Result<()> {
	// -- Setup & Fixtures
	let text = "Data: <ITEM>one</ITEM>, <ITEM key=val>two</ITEM>.";
	let tag_name = "ITEM";

	// -- Exec
	let parts: Vec<PartRef> = TagRefIter::new(text, &[tag_name], true).collect();
	let tags = extract_tag_elem_refs(parts);

	// -- Check
	assert_eq!(tags.len(), 2);
	assert_eq!(
		tags[0],
		TagElemRef {
			tag_name: "ITEM",
			attrs: None,
			content: "one",
			start_idx: 6,
			end_idx: 21,
		}
	);

	let mut expected_attrs = HashMap::new();
	expected_attrs.insert("key", "val");

	assert_eq!(
		tags[1],
		TagElemRef {
			tag_name: "ITEM",
			attrs: Some(expected_attrs),
			content: "two",
			start_idx: 24,
			end_idx: 47,
		}
	);

	Ok(())
}

#[test]
fn test_support_tag_content_iter_no_tags() -> Result<()> {
	// -- Setup & Fixtures
	let text = "Just plain text without any tags.";
	let tag_name = "MARKER";

	// -- Exec
	let parts: Vec<PartRef> = TagRefIter::new(text, &[tag_name], true).collect();
	let tags = extract_tag_elem_refs(parts);

	// -- Check
	assert!(tags.is_empty());

	Ok(())
}

#[test]
fn test_support_tag_content_iter_empty_content() -> Result<()> {
	// -- Setup & Fixtures
	let text = "<EMPTY></EMPTY>";
	let tag_name = "EMPTY";

	// -- Exec
	let parts: Vec<PartRef> = TagRefIter::new(text, &[tag_name], true).collect();
	let tags = extract_tag_elem_refs(parts);

	// -- Check
	assert_eq!(tags.len(), 1);
	assert_eq!(
		tags[0],
		TagElemRef {
			tag_name: "EMPTY",
			attrs: None,
			content: "",
			start_idx: 0,
			end_idx: 14,
		}
	);

	Ok(())
}

#[test]
fn test_support_tag_content_iter_nested_like() -> Result<()> {
	// -- Setup & Fixtures
	let text = "<OUTER>outer <INNER>inner</INNER> outer</OUTER>";
	let tag_name_outer = "OUTER";
	let tag_name_inner = "INNER";

	// -- Exec Outer
	let parts_outer: Vec<PartRef> = TagRefIter::new(text, &[tag_name_outer], true).collect();
	let tags_outer = extract_tag_elem_refs(parts_outer);
	// -- Check Outer
	assert_eq!(tags_outer.len(), 1);
	assert_eq!(
		tags_outer[0],
		TagElemRef {
			tag_name: "OUTER",
			attrs: None,
			content: "outer <INNER>inner</INNER> outer",
			start_idx: 0,
			end_idx: 46,
		}
	);

	// -- Exec Inner
	let parts_inner: Vec<PartRef> = TagRefIter::new(text, &[tag_name_inner], true).collect();
	let tags_inner = extract_tag_elem_refs(parts_inner);
	// -- Check Inner
	assert_eq!(tags_inner.len(), 1);
	assert_eq!(
		tags_inner[0],
		TagElemRef {
			tag_name: "INNER",
			attrs: None,
			content: "inner",
			start_idx: 13,
			end_idx: 32,
		}
	);

	Ok(())
}

#[test]
fn test_support_tag_content_iter_malformed_open() -> Result<()> {
	// -- Setup & Fixtures
	let text = "<MARKER oops </MARKER>"; // Missing '>'
	let tag_name = "MARKER";

	// -- Exec
	let parts: Vec<PartRef> = TagRefIter::new(text, &[tag_name], true).collect();
	let tags = extract_tag_elem_refs(parts);

	// -- Check
	// The current implementation stops if '>' isn't found for the opening tag.
	assert!(tags.is_empty());

	Ok(())
}

#[test]
fn test_support_tag_content_iter_unclosed() -> Result<()> {
	// -- Setup & Fixtures
	let text = "<MARKER>content"; // Missing closing tag
	let tag_name = "MARKER";

	// -- Exec
	let parts: Vec<PartRef> = TagRefIter::new(text, &[tag_name], true).collect();
	let tags = extract_tag_elem_refs(parts);

	// -- Check
	// The current implementation stops if the closing tag isn't found.
	assert!(tags.is_empty());

	Ok(())
}

#[test]
fn test_support_tag_content_iter_edges() -> Result<()> {
	// -- Setup & Fixtures
	let text = "<START>at start</START>middle<END>at end</END>";
	let tag_name_start = "START";
	let tag_name_end = "END";

	// -- Exec Start
	let parts_start: Vec<PartRef> = TagRefIter::new(text, &[tag_name_start], true).collect();
	let tags_start = extract_tag_elem_refs(parts_start);
	// -- Check Start
	assert_eq!(tags_start.len(), 1);
	assert_eq!(
		tags_start[0],
		TagElemRef {
			tag_name: "START",
			attrs: None,
			content: "at start",
			start_idx: 0,
			end_idx: 22,
		}
	);

	// -- Exec End
	let parts_end: Vec<PartRef> = TagRefIter::new(text, &[tag_name_end], true).collect();
	let tags_end = extract_tag_elem_refs(parts_end);
	// -- Check End
	assert_eq!(tags_end.len(), 1);
	assert_eq!(
		tags_end[0],
		TagElemRef {
			tag_name: "END",
			attrs: None,
			content: "at end",
			start_idx: 29,
			end_idx: 45,
		}
	);

	Ok(())
}

#[test]
fn test_support_tag_content_iter_incorrect_tag_name() -> Result<()> {
	// -- Setup & Fixtures
	let text = "<MARKERX>content</MARKERX>";
	let tag_name = "MARKER"; // Searching for MARKER, not MARKERX

	// -- Exec
	let parts: Vec<PartRef> = TagRefIter::new(text, &[tag_name], true).collect();
	let tags = extract_tag_elem_refs(parts);

	// -- Check
	assert!(tags.is_empty());

	Ok(())
}

#[test]
fn test_support_tag_content_iter_tag_name_prefix_check() -> Result<()> {
	// -- Setup & Fixtures
	let text = "<TAG_EXTRA>extra</TAG_EXTRA><TAG>real</TAG>";
	let tag_name = "TAG";

	// -- Exec
	let parts: Vec<PartRef> = TagRefIter::new(text, &[tag_name], true).collect();
	let tags = extract_tag_elem_refs(parts);

	// -- Check
	assert_eq!(tags.len(), 1);
	assert_eq!(
		tags[0],
		TagElemRef {
			tag_name: "TAG",
			attrs: None,
			content: "real",
			start_idx: 28,
			end_idx: 42,
		}
	);

	Ok(())
}

#[test]
fn test_support_tag_content_iter_multiple_tag_names() -> Result<()> {
	// -- Setup & Fixtures
	let text = "Alpha <ONE>first</ONE> Beta <TWO attr=ok>second</TWO> Gamma";
	let tag_names = ["ONE", "TWO"];

	// -- Exec
	let parts: Vec<PartRef> = TagRefIter::new(text, &tag_names, true).collect();
	let tags = extract_tag_elem_refs(parts);

	// -- Check
	assert_eq!(tags.len(), 2);
	assert_eq!(
		tags[0],
		TagElemRef {
			tag_name: "ONE",
			attrs: None,
			content: "first",
			start_idx: 6,
			end_idx: 21,
		}
	);

	let mut expected_attrs = HashMap::new();
	expected_attrs.insert("attr", "ok");

	assert_eq!(
		tags[1],
		TagElemRef {
			tag_name: "TWO",
			attrs: Some(expected_attrs),
			content: "second",
			start_idx: 28,
			end_idx: 52,
		}
	);

	Ok(())
}

// region:    --- PartRef Text Fragment Tests

#[test]
fn test_support_tag_content_iter_partref_simple() -> Result<()> {
	// -- Setup & Fixtures
	let text = "Some text <DATA>content</DATA> more text.";
	let tag_name = "DATA";

	// -- Exec
	let parts: Vec<PartRef> = TagRefIter::new(text, &[tag_name], true).collect();

	// -- Check
	assert_eq!(parts.len(), 3);
	assert_eq!(parts[0], PartRef::Text("Some text "));
	assert!(matches!(parts[1], PartRef::TagElemRef(_)));
	assert_eq!(parts[2], PartRef::Text(" more text."));

	Ok(())
}

#[test]
fn test_support_tag_content_iter_partref_no_tags() -> Result<()> {
	// -- Setup & Fixtures
	let text = "Just plain text without any tags.";
	let tag_name = "MARKER";

	// -- Exec
	let parts: Vec<PartRef> = TagRefIter::new(text, &[tag_name], true).collect();

	// -- Check
	assert_eq!(parts.len(), 1);
	assert_eq!(parts[0], PartRef::Text("Just plain text without any tags."));

	Ok(())
}

#[test]
fn test_support_tag_content_iter_partref_tag_at_start() -> Result<()> {
	// -- Setup & Fixtures
	let text = "<DATA>content</DATA> after";
	let tag_name = "DATA";

	// -- Exec
	let parts: Vec<PartRef> = TagRefIter::new(text, &[tag_name], true).collect();

	// -- Check
	assert_eq!(parts.len(), 2);
	assert!(matches!(parts[0], PartRef::TagElemRef(_)));
	assert_eq!(parts[1], PartRef::Text(" after"));

	Ok(())
}

#[test]
fn test_support_tag_content_iter_partref_tag_at_end() -> Result<()> {
	// -- Setup & Fixtures
	let text = "before <DATA>content</DATA>";
	let tag_name = "DATA";

	// -- Exec
	let parts: Vec<PartRef> = TagRefIter::new(text, &[tag_name], true).collect();

	// -- Check
	assert_eq!(parts.len(), 2);
	assert_eq!(parts[0], PartRef::Text("before "));
	assert!(matches!(parts[1], PartRef::TagElemRef(_)));

	Ok(())
}

#[test]
fn test_support_tag_content_iter_partref_only_tag() -> Result<()> {
	// -- Setup & Fixtures
	let text = "<DATA>content</DATA>";
	let tag_name = "DATA";

	// -- Exec
	let parts: Vec<PartRef> = TagRefIter::new(text, &[tag_name], true).collect();

	// -- Check
	assert_eq!(parts.len(), 1);
	assert!(matches!(parts[0], PartRef::TagElemRef(_)));

	Ok(())
}

#[test]
fn test_support_tag_content_iter_partref_multiple_tags() -> Result<()> {
	// -- Setup & Fixtures
	let text = "A <X>1</X> B <X>2</X> C";
	let tag_name = "X";

	// -- Exec
	let parts: Vec<PartRef> = TagRefIter::new(text, &[tag_name], true).collect();

	// -- Check
	assert_eq!(parts.len(), 5);
	assert_eq!(parts[0], PartRef::Text("A "));
	assert!(matches!(parts[1], PartRef::TagElemRef(_)));
	assert_eq!(parts[2], PartRef::Text(" B "));
	assert!(matches!(parts[3], PartRef::TagElemRef(_)));
	assert_eq!(parts[4], PartRef::Text(" C"));

	Ok(())
}

#[test]
fn test_support_tag_content_iter_partref_adjacent_tags() -> Result<()> {
	// -- Setup & Fixtures
	let text = "<A>1</A><B>2</B>";
	let tag_names = ["A", "B"];

	// -- Exec
	let parts: Vec<PartRef> = TagRefIter::new(text, &tag_names, true).collect();

	// -- Check
	assert_eq!(parts.len(), 2);
	assert!(matches!(parts[0], PartRef::TagElemRef(_)));
	assert!(matches!(parts[1], PartRef::TagElemRef(_)));

	Ok(())
}

#[test]
fn test_support_tag_content_iter_partref_empty_input() -> Result<()> {
	// -- Setup & Fixtures
	let text = "";
	let tag_name = "DATA";

	// -- Exec
	let parts: Vec<PartRef> = TagRefIter::new(text, &[tag_name], true).collect();

	// -- Check
	assert!(parts.is_empty());

	Ok(())
}

// endregion: --- PartRef Text Fragment Tests
