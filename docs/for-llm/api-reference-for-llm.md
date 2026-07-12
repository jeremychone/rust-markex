# markex API Reference (LLM Optimized)

Fast, non-validating markup extractor. Optimized for LLM context.

## Crate Root

```rust
pub type Result<T> = core::result::Result<T, Error>;
pub enum Error { Custom(String), ... }
```

## Module: `markex::tag`

### Core Functions

- `fn extract(input: &str, tag_names: &[&str], capture_text: bool) -> Parts`
- `fn extract_with_fence(input: &str, tag_names: &[&str], capture_text: bool, fence: TagFence) -> Parts`
- `fn extract_with_options(input: &str, tag_names: &[&str], capture_text: bool, options: TagOptions) -> Parts`
- `fn extract_refs<'a>(input: &'a str, tag_names: &[&str], capture_text: bool) -> PartsRef<'a>`
- `fn extract_refs_with_fence<'a>(input: &'a str, tag_names: &[&str], capture_text: bool, fence: TagFence) -> PartsRef<'a>`
- `fn extract_refs_with_options<'a>(input: &'a str, tag_names: &[&str], capture_text: bool, options: TagOptions) -> PartsRef<'a>`

### Custom Fences

**Struct `TagFence`**
```rust
pub struct TagFence {
    pub name: &'static str,
    pub open_delim: &'static str,
    pub close_delim: &'static str,
    pub close_delim_alts: Option<&'static [&'static str]>,
    pub closing_tag_prefix: &'static str,
}
```

`TagFence` configures a matched tag syntax:

- `open_delim`: starts opening and closing tags.
- `close_delim`: ends opening and closing tags.
- `closing_tag_prefix`: appears after `open_delim` in paired closing tags and marks self-closing opening tags.
- `name`: static descriptive identifier, not used for matching.

- `FENCE_XML`: XML-compatible delimiters, such as `<FILE>content</FILE>`.
- `FENCE_BRACKETS`: Triple-square-bracket delimiters for multiline structured content.
`markex` includes [`tag::FENCE_XML`] and [`tag::FENCE_BRACKETS`], and applications can define their own fence values. `FENCE_BRACKETS` accepts both its canonical `]]]` closing delimiter and the tolerant `]]` fallback, including paired and self-closing forms.

Pass a fence to `extract_with_fence`, `extract_refs_with_fence`, or either iterator's `new_with_fence` constructor. Self-closing tags place `closing_tag_prefix` immediately before `close_delim`, such as `<DELETE />` or `[[[DELETE /]]]`.

```rust
use markex::tag::{extract_with_fence, FENCE_BRACKETS};

let input = r#"[[[BIG_CONTENT path="/some/path.txt"]]]
... some big content
[[[/BIG_CONTENT]]]"#;
let parts = extract_with_fence(input, &["BIG_CONTENT"], false, FENCE_BRACKETS);
let file = &parts.tag_elems()[0];

assert_eq!(file.tag, "BIG_CONTENT");
assert_eq!(file.content, "\n... some big content\n");
```

Keeping the tags on their own lines separates directives from large payloads. This style can help LLMs generate tag output without confusing the markup syntax with content.

Custom fences support the same paired and self-closing forms:

```rust
use markex::tag::{extract_with_fence, TagFence};

let fence = TagFence {
    name: "mustache",
    open_delim: "{{",
    close_delim: "}}",
    close_delim_alts: None,
    closing_tag_prefix: "/",
};
let parts = extract_with_fence("{{DATA key=value}}payload{{/DATA}}", &["DATA"], false, fence);

assert_eq!(parts.tag_elems()[0].content, "payload");
```

`FENCE_BRACKETS` also accepts `]]` as a fallback closing delimiter, so a multiline block may use `[[[BIG_CONTENT]]` and `[[[/BIG_CONTENT]]`. Custom [`TagFence`] values can opt into the same tolerant behavior with `close_delim_alts`. The canonical delimiter is preferred whenever it and an alternate begin at the same position.

### Parser Options

**Struct `TagOptions`**
```rust
pub struct TagOptions {
    pub fence: Option<TagFence>,
    pub auto_close: bool,
}
```

`TagOptions::default()` preserves XML-compatible parsing. Use `TagOptions::default().with_fence(FENCE_BRACKETS)` to
configure bracket-tag parsing through `extract_with_options`, `extract_refs_with_options`, `TagIter::new_with_options`,
or `TagRefIter::new_with_options`. Use `TagOptions::default().with_auto_close(true)` to recover an element whose
closing tag is omitted before the next valid configured opening tag. Auto-close is disabled by default, applies to
same-name and different-name configured openings, and does not support nesting.

An element closed this way has `auto_closed: true`; normally closed and self-closing elements have `auto_closed: false`.
Malformed, partial, and non-configured candidate tags do not trigger auto-close. The subsequent valid opening remains
available for normal parsing.

### Owned Types

**Struct `TagElem`**
```rust
pub struct TagElem {
    pub tag: String,
    pub attrs: Option<HashMap<String, String>>,
    pub content: String,
    pub auto_closed: bool,
}
```

**Enum `Part`**
```rust
pub enum Part {
    Text(String),
    TagElem(TagElem),
}
```

**Struct `Parts`**
- `fn parts(&self) -> &Vec<Part>`
- `fn into_parts(self) -> Vec<Part>`
- `fn tag_names(&self) -> Vec<&str>`
- `fn tag_elems(&self) -> Vec<&TagElem>`
- `fn into_tag_elems(self) -> Vec<TagElem>`
- `fn texts(&self) -> Vec<&String>`
- `fn into_texts(self) -> Vec<String>`
- `fn into_with_extrude_content(self) -> (Vec<TagElem>, String)`

### Reference Types (Zero-copy)

**Struct `TagElemRef<'a>`**
```rust
pub struct TagElemRef<'a> {
    pub tag_name: &'a str,
    pub attrs: Option<HashMap<&'a str, &'a str>>,
    pub content: &'a str,
    pub auto_closed: bool,
    pub start_idx: usize,
    pub end_idx: usize,
}
```

**Enum `PartRef<'a>`**
```rust
pub enum PartRef<'a> {
    Text(&'a str),
    TagElemRef(TagElemRef<'a>),
}
```

**Struct `PartsRef<'a>`**
- `fn parts(&self) -> &Vec<PartRef<'a>>`
- `fn into_parts(self) -> Vec<PartRef<'a>>`
- `fn tag_names(&self) -> Vec<&str>`
- `fn tag_elems(&self) -> Vec<&TagElemRef<'a>>`
- `fn texts(&self) -> Vec<&'a str>`

### Iterators

- `TagIter<'a>`: Yields `Part`.
- `TagRefIter<'a>`: Yields `PartRef<'a>`.

**Constructors:**

- `TagIter::new(input: &'a str, tag_names: &[&str], capture_text: bool)`
- `TagIter::new_single_tag(input: &'a str, tag_name: &'a str, capture_text: bool)`
- `TagIter::new_with_fence(input: &'a str, tag_names: &[&str], capture_text: bool, fence: TagFence)`
- `TagIter::new_with_options(input: &'a str, tag_names: &[&str], capture_text: bool, options: TagOptions)`
- `TagRefIter::new(input: &'a str, tag_names: &[&str], capture_text: bool)`
- `TagRefIter::new_with_fence(input: &'a str, tag_names: &[&str], capture_text: bool, fence: TagFence)`
- `TagRefIter::new_with_options(input: &'a str, tag_names: &[&str], capture_text: bool, options: TagOptions)`

`TagIter::new_single_tag` is the owned iterator convenience constructor. Both iterators provide `new_with_fence` and
`new_with_options` for streaming extraction with custom configuration. Pass `TagOptions::with_auto_close(true)` to
either `new_with_options` constructor to enable streaming auto-close recovery.
