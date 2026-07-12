# Owned streaming tag extraction

[`TagIter`] incrementally extracts configured tags from an input string and yields owned [`Part`] values. It is useful when callers need to process extracted text and elements sequentially instead of collecting a [`Parts`] value.

Use [`TagIter::new`] for XML-compatible syntax, or [`TagIter::new_with_fence`] to use a custom [`TagFence`]. The iterator supports configured alternate closing delimiters through the supplied fence, including [`FENCE_BRACKETS`] support for both `]]]` and `]]`.

Set `capture_text` to `true` to receive unmatched spans as [`Part::Text`] values in source order. Set it to `false` to yield only matched tag elements.

Use [`TagIter::new_with_options`] with [`TagOptions::with_auto_close`] to recover an element whose closing tag is omitted before the next valid configured opening tag. Synthesized elements have [`TagElem::auto_closed`] set to `true`.
