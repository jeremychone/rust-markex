# markex

Fast, **non-validating** markup element extractor (XML like tags, Markdown Ref, Code block, Section).

- **Fast Extraction:** Optimized for quickly finding defined elem structures.
- **Elements** Support tag (xml-ish) element, and later Markdown MdCodeBlock, MdSection, MdRef (links).
- **Iterators:** Provides streaming iteration over extracted elements (e.g., `TagElemIter`).
- **Full Content Parsing:** Offer convenient `extract` function that return `ExtractedData` with data consolidation apis. 

**Important** This is not an XML, Markdown, or other format parser. It just extract predefine markup elements from a text content, and ignore any other formatting, 

> Note: for now, only support tag extractor. 

```rust
use std::error::Error;
use markex::tag;

fn main() -> Result<Box dyn<Error>> {
    let input = "Text before <MY_TAG>some content</MY_TAG> and after.";
    
    let extracted_data = tag::extract(input, &["MY_TAG"], true); 
    
    for part in extracted_data.parts {
        match part {
            Part::Text(t) => println!("Text: {t:?}"),
            Part::TagElem(e) => println!("TagElem: {} ({})", e.tag, e.content),
        }
    }
    
    /* Output:
    Text: "Text before "
    TagElem: MY_TAG (some content)
    Text: " and after."
    */
    
    Ok(())
}
```

