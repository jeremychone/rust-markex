//! Provides structures and iterators for extracting content defined by specific tags.

// region:    --- Modules

mod attrs_parser;
mod extract;
mod tag_elem;
mod tag_elem_iter;
mod tag_elem_ref_iter;

use tag_elem_ref_iter::TagElemRefIterator;

pub use extract::{ExtractedData, extract};
pub use tag_elem::*;
pub use tag_elem_iter::*;

// endregion: --- Modules
