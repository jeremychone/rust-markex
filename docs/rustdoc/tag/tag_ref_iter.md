# Borrowed streaming tag extraction

[`TagRefIter`] incrementally extracts configured tags from an input string and yields zero-copy [`PartRef`] values. Its tag names, attributes, text fragments, and element content borrow from the input, so the input must outlive the iterator and yielded values.

Use [`TagRefIter::new`] with `None` for XML-compatible syntax, or pass [`TagOptions::with_fence`] to use a custom [`TagFence`]. The supplied fence controls canonical and alternate closing delimiters. In particular, [`FENCE_BRACKETS`] accepts canonical `]]]` delimiters and tolerant `]]` fallback delimiters.

Use [`TagOptions::with_capture_text`] to receive unmatched spans as [`PartRef::Text`] values in source order. Default options yield only matched tag elements.

Use [`TagRefIter::new`] with [`TagOptions::with_auto_close`] to recover an element whose closing tag is omitted before the next valid configured opening tag. Synthesized elements have [`TagElemRef::auto_closed`] set to `true`.
