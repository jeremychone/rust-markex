[`FENCE_XML`] uses standard XML-compatible delimiters and has no alternatives.

[`FENCE_BRACKETS`] uses `]]]` as its canonical closing delimiter and accepts `]]` as a fallback. Paired tags use an `END_` prefix for their closing tag names:

- canonical: `[[[FILE]]]contents[[[/FILE]]]`
- abbreviated: `[[[FILE]]]contents[[[END_FILE]]`

Self-closing tags use `/` immediately before an accepted closing delimiter. Whitespace before `/` is optional:

- compact: `[[[DELETE path="temp.txt"/]]]`
- spaced: `[[[DELETE path="temp.txt" /]]]`

Alternate delimiters apply consistently to opening tags, paired closing tags, and self-closing tags. Custom fence definitions can opt into the same behavior.

The multiline form keeps tag directives distinct from their payloads, which can help LLMs generate structured output without confusing tags with content.
