use markex::tag::{Part, extract};

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let input = "Text before <MY_TAG>some content</MY_TAG> and after.";

	let extracted_data = extract(input, &["MY_TAG"], true);

	for part in extracted_data.parts {
		match part {
			Part::Text(t) => println!("Text: {t:?}"),
			Part::TagElem(e) => println!("TagElem: {} ({})", e.tag, e.content),
		}
	}

	Ok(())
}
