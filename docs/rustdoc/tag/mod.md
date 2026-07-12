# Tag extraction

This module extracts configured paired and self-closing tags from an input string. It is intentionally non-validating: it recognizes requested structures without attempting to parse or validate an entire markup document.

Use [`extract`] for XML-compatible tags, [`extract_with_fence`] when the input uses a custom [`TagFence`], or
[`extract_with_options`] for extensible parser configuration.

## Extracting elements

```rust
use markex::tag::{self, Part};

let input = "Start <FILE path=\"a.txt\">contents</FILE> end";
let parts = tag::extract(input, &["FILE"], true);

assert!(matches!(parts.parts()[0], Part::Text(_)));
assert_eq!(parts.tag_elems()[0].content, "contents");
```

The `capture_text` argument determines whether unmatched spans are returned as text parts. When enabled, result ordering matches the source input.

Owned extraction returns [`Parts`], containing [`Part::Text`] and [`Part::TagElem`] values. A [`TagElem`] owns its name, attributes, and content.

## Custom fences

A [`TagFence`] describes a tag syntax with:

- `open_delim`, the delimiter starting an opening or closing tag.
- `close_delim`, the delimiter ending an opening or closing tag.
- `close_delim_alts`, optional fallback delimiters accepted in addition to `close_delim`.
- `closing_tag_prefix`, the prefix between `open_delim` and a closing tag name.
- `name`, a descriptive static name for the fence.

[`FENCE_XML`] is the default used by [`extract`] and [`extract_refs`]. [`FENCE_BRACKETS`] recognizes triple-square-bracket tags:

```rust
use markex::tag::{self, FENCE_BRACKETS};

let input = r#"[[[BIG_CONTENT path="/some/path.txt"]]]
... some big content
[[[/BIG_CONTENT]]]"#;

let parts = tag::extract_with_fence(input, &["BIG_CONTENT"], false, FENCE_BRACKETS);

assert_eq!(parts.tag_elems()[0].content, "\n... some big content\n");
```

Place bracket tags on separate lines from large payloads. This makes the structured boundary clear and can help LLMs generate tag output without confusing the tag syntax with content.

`FENCE_BRACKETS` also accepts `]]` as a fallback closing delimiter, including paired and self-closing tags. For example, the opening and closing tags in the preceding multiline block may use `[[[BIG_CONTENT]]` and `[[[/BIG_CONTENT]]`. If its canonical `]]]` delimiter and `]]` alternate both begin at the same location, extraction uses the longer canonical delimiter.

Custom fences may configure the same behavior with `close_delim_alts`. The canonical delimiter is always considered first:

```rust
use markex::tag::{self, TagFence};

let fence = TagFence {
    name: "mustache",
    open_delim: "{{",
    close_delim: "}}",
    close_delim_alts: Some(&["}"]),
    closing_tag_prefix: "/",
};
let parts = tag::extract_with_fence("{{DATA}payload{{/DATA}", &["DATA"], false, fence);

assert_eq!(parts.tag_elems()[0].content, "payload");
```

Use [`extract_refs_with_fence`] for the same syntax when zero-copy results are needed.

## Options

[`TagOptions`] configures optional extraction behavior. Its default value preserves XML-compatible parsing, while
[`TagOptions::with_fence`] selects a custom [`TagFence`]. [`TagOptions::with_auto_close`] opts into recovery when a configured element omits its closing tag before another valid configured opening tag.

```rust
use markex::tag::{self, FENCE_BRACKETS, TagOptions};

let options = TagOptions::default().with_fence(FENCE_BRACKETS);
let parts = tag::extract_with_options("[[[DATA]]]content[[[/DATA]]]", &["DATA"], false, options);

assert_eq!(parts.tag_elems()[0].content, "content");
```

## Auto-close recovery

By default, extraction requires a matching closing tag. With `auto_close` enabled, a configured opening tag that appears before the current element's matching close synthesizes a close for the current element. The following opening tag remains available for normal parsing, and only the synthesized element has `auto_closed: true`.

```rust
use markex::tag::{self, TagOptions};

let options = TagOptions::default().with_auto_close(true);
let parts = tag::extract_with_options("<FILE>first <DATA>second</DATA>", &["FILE", "DATA"], false, options);
let elements = parts.tag_elems();

assert_eq!(elements[0].content, "first ");
assert!(elements[0].auto_closed);
assert!(!elements[1].auto_closed);
```

Auto-close applies to same-name and different-name configured openings, but does not enable nested parsing. Invalid or partial configured-tag candidates do not trigger recovery.

## Borrowed results

[`extract_refs`], [`extract_refs_with_fence`], and [`extract_refs_with_options`] return [`PartsRef`]. Its [`PartRef`]
values and [`TagElemRef`] fields borrow from the original input, avoiding allocation for the extracted text and attribute strings.

The input must outlive the returned `PartsRef`.

## Streaming iterators

[`TagIter`] yields owned [`Part`] values, while [`TagRefIter`] yields borrowed [`PartRef`] values. Both provide `new`,
`new_with_fence`, and `new_with_options` constructors for incremental processing. [`TagIter`] also provides
`new_single_tag` for single-tag owned extraction. Pass [`TagOptions::with_auto_close`] through either iterator's
`new_with_options` constructor to enable streaming auto-close recovery.
