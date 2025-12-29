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
- `fn extract_refs<'a>(input: &'a str, tag_names: &[&str], capture_text: bool) -> PartsRef<'a>`

### Owned Types

**Struct `TagElem`**
```rust
pub struct TagElem {
    pub tag: String,
    pub attrs: Option<HashMap<String, String>>,
    pub content: String,
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
    pub attrs_raw: Option<&'a str>,
    pub content: &'a str,
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
- `new(input: &'a str, tag_names: &[&str], capture_text: bool)`
- `new_single_tag(input: &'a str, tag_name: &str, capture_text: bool)`
