#![doc = include_str!("../../docs/rustdoc/tag/options.md")]

use super::{FENCE_XML, TagFence};

/// Configures optional behavior for tag extraction APIs.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct TagOptions {
	/// The delimiter configuration, or XML-compatible parsing when omitted.
	pub fence: Option<TagFence>,

	/// Whether to synthesize a close before a subsequent configured opening tag.
	pub auto_close: bool,

	/// Whether to include text fragments outside extracted tags.
	pub capture_text: bool,
}

/// Chainable Setters
impl TagOptions {
	/// Sets whether extraction includes text fragments outside extracted tags.
	pub fn with_capture_text(mut self, capture_text: bool) -> Self {
		self.capture_text = capture_text;
		self
	}

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
}

/// Accessors
impl TagOptions {
	pub(crate) fn capture_text(self) -> bool {
		self.capture_text
	}

	pub(crate) fn fence_or_default(self) -> TagFence {
		self.fence.unwrap_or(FENCE_XML)
	}
}

// region:    --- Froms

impl From<Option<TagOptions>> for TagOptions {
	fn from(options: Option<TagOptions>) -> Self {
		options.unwrap_or_default()
	}
}

// endregion: --- Froms

// region:    --- Tests

#[cfg(test)]
mod tests {
	type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>; // For tests.

	use super::*;
	use crate::tag::FENCE_BRACKETS;

	#[test]
	fn test_tag_options_default_text_capture_disabled() -> Result<()> {
		// -- Setup & Fixtures
		let options = TagOptions::default();

		// -- Exec

		// -- Check
		assert!(!options.capture_text);
		assert_eq!(options.fence, None);
		assert!(!options.auto_close);

		Ok(())
	}

	#[test]
	fn test_tag_options_with_capture_text_chained() -> Result<()> {
		// -- Setup & Fixtures

		// -- Exec
		let options = TagOptions::default()
			.with_capture_text(true)
			.with_fence(FENCE_BRACKETS)
			.with_auto_close(true);

		// -- Check
		assert!(options.capture_text);
		assert_eq!(options.fence, Some(FENCE_BRACKETS));
		assert!(options.auto_close);

		Ok(())
	}

	#[test]
	fn test_tag_options_from_option() -> Result<()> {
		// -- Setup & Fixtures
		let configured = TagOptions::default().with_capture_text(true).with_auto_close(true);

		// -- Exec
		let default_options: TagOptions = None.into();
		let configured_options: TagOptions = Some(configured).into();

		// -- Check
		assert_eq!(default_options, TagOptions::default());
		assert_eq!(configured_options, configured);

		Ok(())
	}
}

// endregion: --- Tests
