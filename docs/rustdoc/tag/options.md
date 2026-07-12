# Tag options

[`TagOptions`] configures optional behavior shared by tag extraction APIs. Its default value preserves XML-compatible parsing through [`FENCE_XML`] and requires explicit closing tags.

Use [`TagOptions::with_fence`] to select a different [`TagFence`], such as [`FENCE_BRACKETS`]. The option type is designed to allow additional parser configuration without expanding existing function signatures.

```rust
use markex::tag::{FENCE_BRACKETS, TagOptions};

let options = TagOptions::default().with_fence(FENCE_BRACKETS);
assert_eq!(options.fence, Some(FENCE_BRACKETS));
```

## Auto-close recovery

Use [`TagOptions::with_auto_close`] to opt into recovery for semi-structured input that omits a closing tag before a subsequent configured opening tag. When enabled, the parser synthesizes a close immediately before the next valid configured opening tag and marks the extracted [`TagElem`] or [`TagElemRef`] with `auto_closed: true`.

This mode does not support nesting. For configured tags in `<OUTER><INNER>value</INNER></OUTER>`, `OUTER` is auto-closed before `INNER`. The unmatched trailing `</OUTER>` is not part of either extracted element.

```rust
use markex::tag::{self, TagOptions};

let options = TagOptions::default().with_auto_close(true);
let parts = tag::extract_with_options("<FILE>first <DATA>second</DATA>", &["FILE", "DATA"], false, options);
let elements = parts.tag_elems();

assert_eq!(elements[0].content, "first ");
assert!(elements[0].auto_closed);
assert_eq!(elements[1].content, "second");
assert!(!elements[1].auto_closed);
```

When auto-close is disabled, extraction retains strict behavior. Candidate openings must be valid configured tags, so malformed or partial tag names do not synthesize a closing boundary.

## Text capture

Text fragments outside configured tags are omitted by default. Use [`TagOptions::with_capture_text`] to include them.

```rust
use markex::tag::TagOptions;

let options = TagOptions::default().with_capture_text(true);
```
