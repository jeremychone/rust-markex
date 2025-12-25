use std::collections::HashMap;

/// Parses a raw string of attributes (key=value pairs) into a HashMap.
///
/// Supports unquoted values, single-quoted values, and double-quoted values.
/// Flag attributes (key without value) are stored with an empty string value.
///
/// Examples: `path="a/b.txt" id=123 flag`
pub fn parse_attribute(attrs_raw: Option<&str>) -> Option<HashMap<String, String>> {
	let raw = attrs_raw?.trim();
	if raw.is_empty() {
		return None;
	}

	let chars: Vec<char> = raw.chars().collect();
	let len = chars.len();
	let mut idx = 0;
	let mut attrs = HashMap::new();

	while idx < len {
		while idx < len && chars[idx].is_whitespace() {
			idx += 1;
		}
		if idx >= len {
			break;
		}

		let key_start = idx;
		while idx < len && !chars[idx].is_whitespace() && chars[idx] != '=' {
			idx += 1;
		}

		if key_start == idx {
			idx += 1;
			continue;
		}

		let key: String = chars[key_start..idx].iter().collect();
		let key = key.trim();
		if key.is_empty() {
			continue;
		}
		let key = key.to_string();

		while idx < len && chars[idx].is_whitespace() {
			idx += 1;
		}

		let mut value = String::new();

		if idx < len && chars[idx] == '=' {
			idx += 1;

			while idx < len && chars[idx].is_whitespace() {
				idx += 1;
			}

			if idx < len {
				let current = chars[idx];
				if current == '"' || current == '\'' {
					let quote = current;
					idx += 1;
					let value_start = idx;
					while idx < len && chars[idx] != quote {
						idx += 1;
					}
					value = chars[value_start..idx].iter().collect();
					if idx < len {
						idx += 1;
					}
				} else {
					let value_start = idx;
					while idx < len && !chars[idx].is_whitespace() {
						idx += 1;
					}
					value = chars[value_start..idx].iter().collect();
				}
			}
		}

		attrs.insert(key, value);
	}

	if attrs.is_empty() { None } else { Some(attrs) }
}

// region:    --- Tests

#[path = "attrs_parser_tests.rs"]
#[cfg(test)]
mod tests;

// endregion: --- Tests
