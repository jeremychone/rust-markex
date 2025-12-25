//! Provides structures and iterators for extracting content defined by specific tags.

// region:    --- Modules

mod support;

mod extract;
mod extracted_data;
#[allow(clippy::module_inception)]
mod tag;
mod tag_iter;
mod tag_ref_iter;

use tag_ref_iter::TagElemRefIterator;

pub use extract::extract;
pub use extracted_data::ExtractedData;
pub use tag::*;
pub use tag_iter::*;

// endregion: --- Modules
