# Tag options

[`TagOptions`] configures optional behavior shared by tag extraction APIs. Its default value preserves XML-compatible parsing through [`FENCE_XML`].

Use [`TagOptions::with_fence`] to select a different [`TagFence`], such as [`FENCE_BRACKETS`]. The option type is designed to allow additional parser configuration without expanding existing function signatures.

```rust
use markex::tag::{FENCE_BRACKETS, TagOptions};

let options = TagOptions::default().with_fence(FENCE_BRACKETS);
assert_eq!(options.fence, Some(FENCE_BRACKETS));
```
