#![doc = include_str!("../../docs/rustdoc/tag/options.md")]

use super::{FENCE_XML, TagFence};

/// Configures optional behavior for tag extraction APIs.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct TagOptions {
	/// The delimiter configuration, or XML-compatible parsing when omitted.
	pub fence: Option<TagFence>,

	/// Whether to synthesize a close before a subsequent configured opening tag.
	pub auto_close: bool,
}

impl TagOptions {
	/// Sets the delimiter configuration used for tag extraction.
	pub fn with_fence(mut self, fence: TagFence) -> Self {
		self.fence = Some(fence);
		self
	}

	/// Sets whether extraction may synthesize closing boundaries.
	pub fn with_auto_close(mut self, auto_close: bool) -> Self {
		self.auto_close = auto_close;
		self
	}

	pub(crate) fn fence_or_default(self) -> TagFence {
		self.fence.unwrap_or(FENCE_XML)
	}
}
