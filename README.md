# markex

Fast, **non-validating** markup element extractor for Tag elements (XML-like), and later Markdown elements.

- **Fast Extraction:** Optimized for finding defined element structures without full document parsing.
- **Owned & Borrowed:** Provides both owned (`Parts`) and zero-copy reference (`PartsRef`) extraction.
- **Iterators:** Streaming iteration via `TagIter` and `TagRefIter`.

## Quick Start

```rust
use markex::tag::{self, Part};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = "Text before <DATA id=123>some content</DATA> and after.";
    
    // 1. Owned Extraction
    let parts = tag::extract(input, &["DATA"], true); 
    
    for part in parts {
        match part {
            Part::Text(t) => println!("Text: {t:?}"),
            Part::TagElem(e) => println!("Tag: {} | Content: {}", e.tag, e.content),
        }
    }
    
    // 2. Extrude content (concatenate text, keep elements)
    let (elems, text) = parts.into_with_extrude_content();
    
    Ok(())
}
```

## Zero-copy References

For high-performance scenarios, use `extract_refs` to get `PartRef` which contains slices of the original input.

```rust
use markex::tag;

let input = "<FILE path='a.txt'>content</FILE>";
let parts_ref = tag::extract_refs(input, &["FILE"], true);

for part in parts_ref {
    // Zero-copy slices
}
```

## API Highlights

- `tag::extract(...) -> Parts`: Returns owned data.
- `tag::extract_refs(...) -> PartsRef`: Returns references (zero-copy).
- `Parts / PartsRef`: Collection-like structures with `tag_elems()`, `texts()`, and iteration support.
- `TagIter / TagRefIter`: Lower-level iterators for streaming processing.
