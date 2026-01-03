use std::collections::HashMap;

/// Parses a raw string of attributes (key=value pairs) into a HashMap of references.
pub fn parse_attrs_ref(attrs_raw: Option<&str>) -> Option<HashMap<&str, &str>> {
	let raw = attrs_raw?.trim();
	if raw.is_empty() {
		return None;
	}

	let mut attrs = HashMap::new();
	let mut current = raw;

	while !current.is_empty() {
		current = current.trim_start();
		if current.is_empty() {
			break;
		}

		// Find key
		let key_end = current.find(|c: char| c.is_whitespace() || c == '=').unwrap_or(current.len());
		let key = &current[..key_end];
		current = &current[key_end..].trim_start();

		let mut value = "";
		if current.starts_with('=') {
			current = &current[1..].trim_start();
			if !current.is_empty() {
				let first_char = current.chars().next().unwrap();
				if first_char == '"' || first_char == '\'' {
					let quote = first_char;
					current = &current[1..];
					if let Some(val_end) = current.find(quote) {
						value = &current[..val_end];
						current = &current[val_end + 1..];
					} else {
						// Unclosed quote, take rest as value
						value = current;
						current = "";
					}
				} else {
					let val_end = current.find(|c: char| c.is_whitespace()).unwrap_or(current.len());
					value = &current[..val_end];
					current = &current[val_end..];
				}
			}
		}

		if !key.is_empty() {
			attrs.insert(key, value);
		}
	}

	if attrs.is_empty() { None } else { Some(attrs) }
}

// region:    --- Tests

#[path = "attrs_parser_tests.rs"]
#[cfg(test)]
mod tests;

// endregion: --- Tests
