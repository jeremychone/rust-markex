#![doc = include_str!("../../docs/rustdoc/tag/options.md")]

use super::{FENCE_XML, TagFence};

/// Configures optional behavior for tag extraction APIs.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct TagOptions {
	/// The delimiter configuration, or XML-compatible parsing when omitted.
	pub fence: Option<TagFence>,
}

impl TagOptions {
	/// Sets the delimiter configuration used for tag extraction.
	pub fn with_fence(mut self, fence: TagFence) -> Self {
		self.fence = Some(fence);
		self
	}

	pub(crate) fn fence_or_default(self) -> TagFence {
		self.fence.unwrap_or(FENCE_XML)
	}
}
