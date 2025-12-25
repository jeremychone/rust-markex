//! Provides structures and iterators for extracting content defined by specific tags.

// region:    --- Modules

mod support;

mod extract;
mod parts;
mod parts_ref;
#[allow(clippy::module_inception)]
mod tag;
mod tag_iter;
mod tag_ref;
mod tag_ref_iter;

pub use extract::extract;
pub use extract::extract_refs;
pub use parts::Parts;
pub use parts::*;
pub use parts_ref::*;
pub use tag::*;
pub use tag_iter::*;
pub use tag_ref::*;
pub use tag_ref_iter::*;

// endregion: --- Modules
