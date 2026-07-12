use markex::tag::{Part, TagOptions, extract};

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let input = "Text before <MY_TAG>some content</MY_TAG> and after.";

	let parts = extract(input, &["MY_TAG"], TagOptions::default().with_capture_text(true));

	for part in parts {
		match part {
			Part::Text(t) => println!("Text: {t:?}"),
			Part::TagElem(e) => println!("TagElem: {} ({})", e.tag, e.content),
		}
	}

	Ok(())
}
