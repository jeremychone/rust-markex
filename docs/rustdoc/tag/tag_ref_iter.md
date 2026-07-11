# Borrowed streaming tag extraction

[`TagRefIter`] incrementally extracts configured tags from an input string and yields zero-copy [`PartRef`] values. Its tag names, attributes, text fragments, and element content borrow from the input, so the input must outlive the iterator and yielded values.

Use [`TagRefIter::new`] for XML-compatible syntax, or [`TagRefIter::new_with_fence`] to use a custom [`TagFence`]. The supplied fence controls canonical and alternate closing delimiters. In particular, [`FENCE_BRACKETS`] accepts canonical `]]]` delimiters and tolerant `]]` fallback delimiters.

Set `capture_text` to `true` to receive unmatched spans as [`PartRef::Text`] values in source order. Set it to `false` to yield only matched tag elements.
