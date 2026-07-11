[`FENCE_XML`] uses standard XML-compatible delimiters and has no alternatives. 

[`FENCE_BRACKETS`] uses `]]]` as its canonical closing delimiter and accepts `]]` as a fallback, allowing both of these paired forms:

- canonical: `[[[FILE]]]contents[[[/FILE]]]`
- abreviated: `[[FILE]]]contents[[[/FILE]]` (some smaller models will generate those even if instructions say three `]]]`)

Alternate delimiters apply consistently to opening tags, paired closing tags, and self-closing tags. Custom fence definitions can opt into the same behavior:

The multiline form keeps tag directives distinct from their payloads, which can help LLMs generate structured output without confusing tags with content.
