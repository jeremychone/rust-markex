## Manifest Configuration

Add the following to your `Cargo.toml` dependencies:

```toml
[dependencies]
markex = "0.1" 
```

## Public API Reference (`markex`)

The primary goal of `markex` is fast, non-validating extraction of structured elements (like tags) from plain text input. Tags are case-sensitive (e.g., `<DATA>...</DATA>`).

### Error Handling

The crate defines standard `Error` and `Result` types for operation consistency.

```rust
pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, derive_more::Display, derive_more::From)]
pub enum Error { 
    // ... custom and external errors
}
```

### Module `markex::tag`

Exports all core parsing structures and functions.

#### Struct `TagElem`

Represents a single extracted element (`<TAG attr=val>content</TAG>`).

```rust
pub struct TagElem {
	pub tag: String, 
	pub attrs: Option<std::collections::HashMap<String, String>>, // Parsed attributes
	pub content: String,
}
```
**Constructor:** `TagElem::new(name, attrs, content)`

#### Enum `Part`

Represents a fragment of the parsed content returned by `parse()`.

```rust
pub enum Part {
	Text(String),
	TagElem(TagElem),
}
```

#### Function `extract`

```rust
pub fn extract(input: &str, tag_names: &[&str], capture_text: bool) -> Parts
```
Parses `input` searching for balanced tags defined in `tag_names`.
- If `capture_text` is true, returns all input fragments (`Part::Text` and `Part::TagElem`).
- If false, only returns `Part::TagElem`.

#### Struct `Parts`

The result structure returned by `extract()`.

```rust
pub struct Parts {
	// ... internal fields
}
```

**Methods on `Parts` (use `use markex::tag::Parts;`):**

- `pub fn parts(&self) -> &Vec<Part>`: Returns references to all parts.
- `pub fn into_parts(self) -> Vec<Part>`: Consumes `self` and returns the vector of parts.
- `pub fn tag_names(&self) -> Vec<&str>`: Returns unique tag names found.
- `pub fn tag_elems(&self) -> Vec<&TagElem>`: Returns references to all extracted `TagElem`s.
- `pub fn into_tag_elems(self) -> Vec<TagElem>`: Consumes `self`, returns all `TagElem`s.
- `pub fn texts(&self) -> Vec<&String>`: Returns references to all text fragments.
- `pub fn into_texts(self) -> Vec<String>`: Consumes `self`, returns all text fragments.
- `pub fn into_with_extrude_content(self) -> (Vec<TagElem>, String)`: Consumes `self`, returns all `TagElem`s and a single concatenated string of all text fragments.

#### Iterator `TagElemIter`

An iterator that yields owned `Part` objects (`Text` or `TagElem`), providing the full sequence of fragments in the input string.
If you only need `TagElem`s, you can filter this iterator, or use `extract` and then `ExtractedData::into_tag_elems`.

```rust
pub struct TagElemIter<'a> // Implements Iterator<Item = Part>
```
**Constructors (use `use markex::tag::TagElemIter;`):**

- `pub fn new(input: &'a str, tag_names: &[&'a str], capture_text: bool) -> Self`: Iterates over multiple tag names.
- `pub fn new_single_tag(input: &'a str, tag_name: &'a str, capture_text: bool) -> Self`: Convenience for a single tag name.

### Example Usage

```rust
use markex::{Result, tag::{extract, TagElemIter, Part}};

fn main() -> Result<()> {
    let input = "Text before <FILE path=\"config.txt\">data</FILE> and after.";
    let tag_name = "FILE";

    // 1. Simple iteration using TagElemIter
    for part in TagElemIter::new_single_tag(input, tag_name, true) {
        match part {
            Part::Text(t) => println!("Text: {t:?}"),
            Part::TagElem(e) => {
                println!("Found tag: {e.tag}");
                if let Some(path) = e.attrs.and_then(|a| a.get("path").cloned()) {
                    println!("  Path: {path}");
                }
                println!("  Content: {e.content}");
            }
        }
    }
    
    // 2. Detailed parsing using extract() to capture all fragments
    let extracted_data = extract(input, &[tag_name], true); 
    
    for part in extracted_data {
        match part {
            Part::Text(t) => println!("Text: {t:?}"),
            Part::TagElem(e) => println!("TagElem: {e.tag} ({e.content})"),
        }
    }

    // 3. Extruding content
    let extracted_data_2 = extract(input, &[tag_name], true); 
    let (tag_elems, text_content) = extracted_data_2.into_with_extrude_content();
    
    println!("Extruded Text: {text_content:?}");
    println!("Found {} elements.", tag_elems.len());

    Ok(())
}
```
