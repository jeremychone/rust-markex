# markex

`markex` is a fast, non-validating Rust extractor for structured tag elements embedded in text. It finds the elements you care about without building a complete document tree.

- Extract XML-like tags or configure a custom delimiter fence.
- Choose owned results with `Parts` or zero-copy borrowed results with `PartsRef`.
- Stream results directly with `TagIter` and `TagRefIter`.

## Quick Start

```rust
use markex::tag::{self, Part, TagOptions};

fn main() {
    let input = r#"Text before
[[[BIG_CONTENT path="/some/path.txt"]]]
... some big content
[[[/BIG_CONTENT]]]
Text after."#;

    // 1. Owned Extraction
    let parts = tag::extract(
        input,
        &["BIG_CONTENT"],
        TagOptions::default()
            .with_capture_text(true)
            .with_fence(tag::FENCE_BRACKETS),
    );

    for part in &parts {
        match part {
            Part::Text(t) => println!("Text: {t:?}"),
            Part::TagElem(e) => println!("Tag: {} | Content: {}", e.tag, e.content),
        }
    }

    // 2. Extrude content (concatenate text, keep elements)
    let (elems, text) = parts.into_with_extrude_content();
    println!("Extracted elements: {}", elems.len());
    println!("Remaining text: {text:?}");
}
```

This prints:

```text
Text: "Text before\n"
Tag: BIG_CONTENT | Content:
... some big content

Text: "\nText after."
Extracted elements: 1
Remaining text: "Text before\n\nText after."
```

The multiline bracket-fence style keeps large tagged content visually separate from its directives. This can help LLMs generate structured tag output without confusing the tag syntax with the content.

## Zero-copy References

For high-performance scenarios, use `extract_refs` to get `PartRef` values that borrow slices from the original input.

```rust
use markex::tag::{self, PartRef, TagOptions};

let input = r#"[[[BIG_CONTENT path="/some/path.txt"]]]
... some big content
[[[/BIG_CONTENT]]]"#;
let parts_ref = tag::extract_refs(
    input,
    &["BIG_CONTENT"],
    TagOptions::default()
        .with_capture_text(true)
        .with_fence(tag::FENCE_BRACKETS),
);

for part in parts_ref {
    match part {
        PartRef::Text(text) => println!("Text: {text:?}"),
        PartRef::TagElemRef(file) => println!("File: {} | Content: {}", file.tag_name, file.content),
    }
}
```

## API Highlights

- `tag::extract(..., options) -> Parts`: Returns owned data.
- `tag::extract_refs(..., options) -> PartsRef`: Returns references (zero-copy).
- `Parts / PartsRef`: Collection-like structures with `tag_elems()`, `texts()`, and iteration support.
- `TagIter / TagRefIter`: Lower-level iterators for streaming processing.

---

[This repo](https://github.com/zcoder-run/rust-markex)