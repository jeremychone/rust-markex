# Owned streaming tag extraction

[`TagIter`] incrementally extracts configured tags from an input string and yields owned [`Part`] values. It is useful when callers need to process extracted text and elements sequentially instead of collecting a [`Parts`] value.

Use [`TagIter::new`] with `None` for XML-compatible syntax, or pass [`TagOptions::with_fence`] to use a custom [`TagFence`]. The iterator supports configured alternate closing delimiters through the supplied fence, including [`FENCE_BRACKETS`] support for both `]]]` and `]]`.

Use [`TagOptions::with_capture_text`] to receive unmatched spans as [`Part::Text`] values in source order. Default options yield only matched tag elements.

Use [`TagIter::new`] with [`TagOptions::with_auto_close`] to recover an element whose closing tag is omitted before the next valid configured opening tag. Synthesized elements have [`TagElem::auto_closed`] set to `true`.
