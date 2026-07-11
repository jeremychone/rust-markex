//! Defines delimiter configurations for tag extraction.
#![doc = include_str!("../../docs/rustdoc/tag/fence.md")]

/// A delimiter configuration used to parse tagged elements.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TagFence {
	/// A descriptive name for the fence configuration.
	pub name: &'static str,
	/// The delimiter that starts an opening or closing tag.
	pub open_delim: &'static str,
	/// The delimiter that ends an opening or closing tag.
	pub close_delim: &'static str,
	/// Optional fallback delimiters accepted in addition to `close_delim`.
	pub close_delim_alts: Option<&'static [&'static str]>,
	/// The prefix between the opening delimiter and a closing tag name.
	pub closing_tag_prefix: &'static str,
}

/// The XML-compatible fence used by the existing extraction APIs.
pub const FENCE_XML: TagFence = TagFence {
	name: "xml",
	open_delim: "<",
	close_delim: ">",
	close_delim_alts: None,
	closing_tag_prefix: "/",
};

/// A triple-square-bracket fence for clearly separating structured payloads.
pub const FENCE_BRACKETS: TagFence = TagFence {
	name: "brackets",
	open_delim: "[[[",
	close_delim: "]]]",
	close_delim_alts: Some(&["]]"]),
	closing_tag_prefix: "/",
};
