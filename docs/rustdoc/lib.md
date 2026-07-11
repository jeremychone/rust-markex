# markex

`markex` is a fast, non-validating extractor for structured markup embedded in text. It finds configured elements without building a complete document tree, making it useful when an application needs known payloads such as file directives, tool calls, or metadata blocks.

The primary API is [`tag`]. It supports XML-compatible tags by default, configurable delimiter fences, owned results, and zero-copy borrowed results.

## Quick start

Extract XML-compatible elements and preserve the text that surrounds them:

```rust
use markex::tag::{self, Part};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = "Before <DATA id=123>payload</DATA> after.";
    let parts = tag::extract(input, &["DATA"], true);

    for part in parts {
        match part {
            Part::Text(text) => println!("Text: {text:?}"),
            Part::TagElem(element) => {
                println!("Tag: {}", element.tag);
                println!("Content: {}", element.content);
            }
        }
    }

    Ok(())
}
```

`capture_text` controls whether text outside matched elements is emitted as `Part::Text`. Set it to `false` when only extracted elements are needed.

## Custom fences

Use a [`tag::TagFence`] when the structured payload uses delimiters other than XML. `markex` includes [`tag::FENCE_XML`] and [`tag::FENCE_BRACKETS`], and applications can define their own fence values.

```rust
use markex::tag::{self, FENCE_BRACKETS};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = "[[[FILE path=\"notes.txt\"]]]contents[[[/FILE]]]";
    let parts = tag::extract_with_fence(input, &["FILE"], false, FENCE_BRACKETS);
    let file = parts
        .into_tag_elems()
        .into_iter()
        .next()
        .ok_or("expected a FILE element")?;

    assert_eq!(file.tag, "FILE");
    assert_eq!(file.content, "contents");

    Ok(())
}
```

A custom fence defines the opening delimiter, closing delimiter, and prefix that identifies closing tags:

```rust
use markex::tag::{self, TagFence};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let fence = TagFence {
        name: "mustache",
        open_delim: "{{",
        close_delim: "}}",
        closing_tag_prefix: "/",
    };
    let parts = tag::extract_with_fence("{{DATA}}value{{/DATA}}", &["DATA"], false, fence);
    let element = parts
        .into_tag_elems()
        .into_iter()
        .next()
        .ok_or("expected a DATA element")?;

    assert_eq!(element.content, "value");

    Ok(())
}
```

## Owned and borrowed extraction

[`tag::extract`] and [`tag::extract_with_fence`] return owned [`tag::Parts`] values. Use [`tag::extract_refs`] or [`tag::extract_refs_with_fence`] when the input must remain available and allocations for extracted strings should be avoided. These return [`tag::PartsRef`], whose text, tag names, attributes, and content borrow from the input.

For streaming processing, use [`tag::TagIter`] or [`tag::TagRefIter`] directly.
