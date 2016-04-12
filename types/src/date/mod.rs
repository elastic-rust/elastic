//! Implementation of the Elasticsearch `date` type.
//!
//! Dates in Elasticsearch are exposed as a formatted `string` which can contain a `date` and/or a `time` component.
//!
//! All dates used by `elastic_types` are expected to be given in `UTC`, and if no time is supplied, then 12:00am will be used instead.
//! Where performance is paramount, the `EpochMillis` date format will parse and format dates the fastest.
//!
//! Because date conversion needs to be done by the `caller`, the `Format` is a first-class citizen in the `ElasticDate` design.
//!
//! # Examples
//!
//! Map with a default `date`:
//!
//! ```
//! # use elastic_types::date::prelude::*;
//! struct MyType {
//! 	pub field: ElasticDate
//! }
//! ```
//!
//! Map with a custom `date`:
//!
//! ```
//! # extern crate serde;
//! # #[macro_use]
//! # extern crate elastic_types;
//! # use std::marker::PhantomData;
//! # fn main() {
//! # use elastic_types::mapping::prelude::*;
//! # use elastic_types::date::prelude::*;
//! # #[derive(Debug, Default, Clone, Copy)]
//! # pub struct MyDateMapping;
//! # impl ElasticDateMapping<EpochMillis> for MyDateMapping { }
//! # impl_date_mapping!(MyDateMapping, EpochMillis);
//! struct MyType {
//! 	pub field: ElasticDate<EpochMillis, MyDateMapping>
//! }
//! # }
//! ```
//!
//! # Links
//! - [Elasticsearch Doc](https://www.elastic.co/guide/en/elasticsearch/reference/current/date.html)

mod format;
mod date;
mod formats;

pub mod mapping;
pub use self::format::*;
pub use self::date::*;
pub use self::formats::*;

use chrono;
use chrono::UTC;

/// A re-export of the `chrono::DateTime` struct with `UTC` timezone.
pub type DT = chrono::DateTime<UTC>;

/// The default date format.
pub type DefaultFormat = BasicDateTime;

pub mod prelude {
	//! Includes non-mapping types for the `date` type.
    //!
    //! This is a convenience module to make it easy to build mappings for multiple types without too many `use` statements.

	pub use super::format::*;
	pub use super::date::*;
	pub use super::formats::*;
}
