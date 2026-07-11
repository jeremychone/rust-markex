# Tag fences

[`TagFence`] configures the delimiters recognized by tag extraction APIs. Pass a fence to [`crate::tag::extract_with_fence`], [`crate::tag::extract_refs_with_fence`], [`crate::tag::TagIter::new_with_fence`], or [`crate::tag::TagRefIter::new_with_fence`].

The canonical `close_delim` ends opening and closing tags. `close_delim_alts` optionally supplies tolerant fallback delimiters. The parser chooses the earliest matching delimiter, and when multiple configured delimiters begin at the same byte position, it chooses the longest one. This preserves canonical syntax when a fallback is its prefix.

[`FENCE_XML`] uses standard XML-compatible delimiters and has no alternatives. [`FENCE_BRACKETS`] uses `]]]` as its canonical closing delimiter and accepts `]]` as a fallback, allowing both of these paired forms:

```text
[[[FILE]]]contents[[[/FILE]]]
[[[FILE]]contents[[[/FILE]]
```

Alternate delimiters apply consistently to opening tags, paired closing tags, and self-closing tags. Custom fence definitions can opt into the same behavior:

```rust
use markex::tag::TagFence;

let fence = TagFence {
    name: "mustache",
    open_delim: "{{",
    close_delim: "}}",
    close_delim_alts: Some(&["}"]),
    closing_tag_prefix: "/",
};
```
