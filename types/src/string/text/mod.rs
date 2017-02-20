//! Implementation of the Elasticsearch `text` type.
//!
//! Text fields are stored as a sequence of tokens, constructed based on the given `analyzer`.
//! They're useful for blobs of content that can be sliced in various ways, like prose.

#[macro_use]
pub mod mapping;
mod text;

pub use self::text::*;

pub mod prelude {
    //! Includes all types for the `text` type.
    //!
    //! This is a convenience module to make it easy to build mappings for multiple types without too many `use` statements.

    pub use super::text::*;
    pub use super::mapping::*;
}
