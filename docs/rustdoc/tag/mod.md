# Tag extraction

This module extracts configured paired and self-closing tags from an input string. It is intentionally non-validating: it recognizes requested structures without attempting to parse or validate an entire markup document.

Use [`extract`] for XML-compatible tags, or [`extract_with_fence`] when the input uses a custom [`TagFence`].

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

let input = "[[[DATA]]]payload[[[/DATA]]]";
let parts = tag::extract_with_fence(input, &["DATA"], false, FENCE_BRACKETS);

assert_eq!(parts.tag_elems()[0].content, "payload");
```

`FENCE_BRACKETS` also accepts `]]` as a fallback closing delimiter, including paired and self-closing tags. For example, `[[[DATA]]payload[[[/DATA]]` is valid. If its canonical `]]]` delimiter and `]]` alternate both begin at the same location, extraction uses the longer canonical delimiter.

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

## Borrowed results

[`extract_refs`] and [`extract_refs_with_fence`] return [`PartsRef`]. Its [`PartRef`] values and [`TagElemRef`] fields borrow from the original input, avoiding allocation for the extracted text and attribute strings.

The input must outlive the returned `PartsRef`.

## Streaming iterators

[`TagIter`] yields owned [`Part`] values, while [`TagRefIter`] yields borrowed [`PartRef`] values. Both provide `new`, `new_single_tag`, and `new_with_fence` constructors for incremental processing.
