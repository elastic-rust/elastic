//! Elasticsearch API Client
//!
//! A client for Elasticsearch.
//! It's mostly a meta-package for a number of smaller pieces including:
//! 
//! - [`elastic_reqwest`] HTTP client
//! - [`elastic_requests`] API request builders
//! - [`elastic_responses`] API response parser
//! - [`elastic_types`] tools for document and mapping APIs
//! 
//! This crate glues these libraries together with some simple assumptions
//! about how they're going to be used.
//! 
//! # Supported Versions
//!
//!  `elastic`       | Elasticsearch
//!  --------------- | -------------
//!  `0.x`           | `5.x`
//! 
//! # Usage
//! 
//! This crate is on [crates.io](https://crates.io/crates/elastic).
//! To get stated, add `elastic` to your `Cargo.toml`:
//! 
//! ```ignore
//! [dependencies]
//! elastic = "*"
//! ```
//! 
//! Then reference in your crate root:
//!
//! ```
//! extern crate elastic;
//! ```
//! 
//! # Examples
//! 
//! ## Making requests
//! 
//! ```no_run
//! use elastic::prelude::*;
//! 
//! let client = Client::new(RequestParams::default()).unwrap();
//! 
//! let req = PingRequest::new();
//! let response = client.request(req).send().unwrap();
//! ```
//! 
//! ## Configuring requests
//! 
//! Create a set of request parameters that are passed to each request:
//! 
//! ```no_run
//! # use elastic::prelude::*;
//! let params = RequestParams::new("http://es_host:9200")
//!     .url_param("pretty", true);
//! 
//! let client = Client::new(params).unwrap();
//! ```
//! 
//! Requests can override parameter values:
//! 
//! ```no_run
//! # use elastic::prelude::*;
//! # let params = RequestParams::new("http://es_host:9200")
//! #     .url_param("pretty", true);
//! # let client = Client::new(params).unwrap();
//! # let req = PingRequest::new();
//! let response = client.request(req)
//!     .params(|p| p.url_param("pretty", false))
//!     .send()
//!     .unwrap();
//! ```
//! 
//! ## Defining document types
//! 
//! Derive `Serialize`, `Deserialize` and `ElasticType` on your document types:
//! 
//! ```no_run
//! extern crate serde;
//! #[macro_use]
//! extern crate serde_derive;
//! #[macro_use]
//! extern crate elastic_types_derive;
//! # extern crate elastic;
//! # use elastic::prelude::*;
//! 
//! # fn main() {
//! #[derive(Serialize, Deserialize, ElasticType)]
//! struct MyType {
//!     pub id: i32,
//!     pub title: String,
//!     pub timestamp: Date<DefaultDateFormat>
//! }
//! # }
//! ```
//! 
//! Use your document type to build index requests:
//! 
//! ```
//! # extern crate serde;
//! # #[macro_use]
//! # extern crate serde_derive;
//! # #[macro_use]
//! # extern crate elastic_types_derive;
//! # extern crate elastic;
//! # use elastic::prelude::*;
//! # fn main() {
//! # #[derive(Serialize, Deserialize, ElasticType)]
//! # struct MyType {
//! #     pub id: i32,
//! #     pub title: String,
//! #     pub timestamp: Date<DefaultDateFormat>
//! # }
//! let doc = MyType {
//!     id: 1,
//!     title: String::from("A title"),
//!     timestamp: Date::now()
//! };
//! 
//! let req = IndexRequest::try_for_doc((Index::from("index"), Id::from(doc.id.to_string()), &doc)).unwrap();
//! # }
//! ```
//! 
//! Use your document type to build mapping requests:
//! 
//! ```
//! # extern crate serde;
//! # #[macro_use]
//! # extern crate serde_derive;
//! # #[macro_use]
//! # extern crate elastic_types_derive;
//! # extern crate elastic;
//! # use elastic::prelude::*;
//! # fn main() {
//! # #[derive(Serialize, Deserialize, ElasticType)]
//! # struct MyType {
//! #     pub id: i32,
//! #     pub title: String,
//! #     pub timestamp: Date<DefaultDateFormat>
//! # }
//! let req = IndicesPutMappingRequest::try_for_mapping((Index::from("index"), MyType::mapping())).unwrap();
//! # }
//! ```

#[macro_use]
extern crate error_chain;
extern crate serde;
extern crate serde_json;
extern crate reqwest;
extern crate elastic_reqwest;
extern crate elastic_requests;
extern crate elastic_types;
extern crate elastic_responses;

mod http_client;
mod impls;

pub mod errors;

pub mod client {
    //! HTTP client, requests and responses.
    //!
    //! This module contains the HTTP client, as well
    //! as request and response types.

    pub use super::http_client::*;
    pub use super::impls::*;
}

pub mod types {
    //! Indexable documents and type mapping.
    //!
    //! This module contains tools for defining Elasticsearch-compatible
    //! document types.

    pub use elastic_types::*;
}

pub mod prelude {
    pub use super::client::*;
    pub use super::types::prelude::*;
}