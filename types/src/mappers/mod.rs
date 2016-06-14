//! Helper mappers for `ElasticType`.
//!
//! Mapping for types is inferred from the generic mapping parameters on `ElasticType`.
//! There are two mappers provided:
//!
//! - `TypeMapper` for mapping user-defined types for the [Put Mapping API](https://www.elastic.co/guide/en/elasticsearch/reference/current/indices-put-mapping.html).
//! - `RsesMapper` for mapping with [`rs-es`](http://benashford.github.io/rs-es/rs_es/index.html).
//!
//! # Examples
//!
//! Any type that derives `ElasticType` can be mapped using one of the various mappers.
//!
//! ## Mapping to a json string
//!
//! ```
//! # #![feature(plugin, custom_derive)]
//! # #![plugin(json_str, elastic_types_macros)]
//! # #[macro_use]
//! # extern crate elastic_types;
//! # extern crate serde;
//! # extern crate serde_json;
//! # use serde::{ Serialize, Deserialize };
//! use elastic_types::mapping::prelude::*;
//!
//! # use elastic_types::date::prelude::*;
//! # #[derive(Default, Clone, Serialize, Deserialize, ElasticType)]
//! # pub struct MyType {
//! # 	pub my_date: ElasticDate<DefaultDateFormat>,
//! # 	pub my_string: String,
//! # 	pub my_num: i32
//! # }
//! # impl serde::Serialize for MyType {
//! # 	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error> where S: serde::Serializer {
//! # 		unimplemented!()
//! # 	}
//! # }
//! # impl serde::Deserialize for MyType {
//! # 	 fn deserialize<D>(deserializer: &mut D) -> Result<Self, D::Error> where D: serde::Deserializer {
//! # 		unimplemented!()
//! # 	}
//! # }
//! # fn main() {
//! let ser = TypeMapper::to_string(MyTypeMapping).unwrap();
//! # }
//! ```
//!
//! ## Mapping to `rs-es`
//!
//! ```no_run
//! # #![feature(plugin, custom_derive)]
//! # #![plugin(json_str, elastic_types_macros)]
//! # #[macro_use]
//! # extern crate elastic_types;
//! # extern crate serde;
//! # use serde::{ Serialize, Deserialize };
//! use elastic_types::mapping::prelude::*;
//!
//! # use elastic_types::date::prelude::*;
//! # #[derive(Default, Clone, Serialize, Deserialize, ElasticType)]
//! # pub struct MyType {
//! # 	pub my_date: ElasticDate<DefaultDateFormat>,
//! # 	pub my_string: String,
//! # 	pub my_num: i32
//! # }
//! # impl serde::Serialize for MyType {
//! # 	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error> where S: serde::Serializer {
//! # 		unimplemented!()
//! # 	}
//! # }
//! # impl serde::Deserialize for MyType {
//! # 	 fn deserialize<D>(deserializer: &mut D) -> Result<Self, D::Error> where D: serde::Deserializer {
//! # 		unimplemented!()
//! # 	}
//! # }
//! # fn main() {
//! let mut writer = RsesSerializer::default();
//! let mut ser = RsesMapper::to_value(MyTypeMapping, &mut writer);
//!
//! //Map `ser` with `rs_es::Client`
//! # }
//! ```

use std::error::Error;
use std::marker::PhantomData;
use serde;
use serde::ser::Error as SerError;
use serde_json;
use ::mapping::{ ElasticFieldMapping, ElasticTypeVisitor };
use ::object::ElasticUserTypeMapping;

pub mod rs_es_map;

pub use self::rs_es_map::{ RsesMapper, Serializer as RsesSerializer, Error as RsesError };

/// Helper for mapping user-defined types.
///
/// This mapper is designed to take a given user-defined type and pass it around to various visitors to map fields.
pub struct TypeMapper<M> where
M: ElasticUserTypeMapping {
	phantom_m: PhantomData<M>
}
impl <M> TypeMapper<M> where
M: ElasticUserTypeMapping {
	/// Map a user-defined type with a given `Serializer`.
	///
	/// The mapping is emitted as a json field, where the key is the name of the type, as defined by `M::data_type()`.
	///
	/// # Examples
	///
	/// ```
	/// # #![feature(plugin, custom_derive)]
	/// # #![plugin(json_str, elastic_types_macros)]
	/// # #[macro_use]
	/// # extern crate elastic_types;
	/// # extern crate serde;
	/// # extern crate serde_json;
	/// # use serde::{ Serialize, Deserialize };
	/// # use elastic_types::mapping::prelude::*;
	/// # use elastic_types::date::prelude::*;
	/// # #[derive(Default, Clone, Serialize, Deserialize, ElasticType)]
	/// # pub struct MyType {
	/// # 	pub my_date: ElasticDate<DefaultDateFormat>,
	/// # 	pub my_string: String,
	/// # 	pub my_num: i32
	/// # }
	/// # impl serde::Serialize for MyType {
	/// # 	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error> where S: serde::Serializer {
	/// # 		unimplemented!()
	/// # 	}
	/// # }
	/// # impl serde::Deserialize for MyType {
	/// # 	 fn deserialize<D>(deserializer: &mut D) -> Result<Self, D::Error> where D: serde::Deserializer {
	/// # 		unimplemented!()
	/// # 	}
	/// # }
	/// # fn main() {
	/// let mut writer = Vec::new();
	/// let mut ser = serde_json::Serializer::new(&mut writer);
	/// let ser = TypeMapper::to_writer(MyTypeMapping, &mut ser).unwrap();
	/// # }
	/// ```
	pub fn to_writer<S>(_: M, serializer: &mut S) -> Result<(), S::Error> where
	S: serde::Serializer {
		serializer.serialize_struct(
			<M as ElasticFieldMapping<()>>::data_type(),
			<M as ElasticUserTypeMapping>::Visitor::new()
		)
	}

	/// Map a user-defined type to a `String`.
	///
	/// The mapping is emitted as a json field, where the key is the name of the type, as defined by `M::data_type()`.
	///
	/// # Examples
	///
	/// ```
	/// # #![feature(plugin, custom_derive)]
	/// # #![plugin(json_str, elastic_types_macros)]
	/// # #[macro_use]
	/// # extern crate elastic_types;
	/// # extern crate serde;
	/// # extern crate serde_json;
	/// # use serde::{ Serialize, Deserialize };
	/// # use elastic_types::mapping::prelude::*;
	/// # use elastic_types::date::prelude::*;
	/// # #[derive(Default, Clone, Serialize, Deserialize, ElasticType)]
	/// # pub struct MyType {
	/// # 	pub my_date: ElasticDate<DefaultDateFormat>,
	/// # 	pub my_string: String,
	/// # 	pub my_num: i32
	/// # }
	/// # impl serde::Serialize for MyType {
	/// # 	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error> where S: serde::Serializer {
	/// # 		unimplemented!()
	/// # 	}
	/// # }
	/// # impl serde::Deserialize for MyType {
	/// # 	 fn deserialize<D>(deserializer: &mut D) -> Result<Self, D::Error> where D: serde::Deserializer {
	/// # 		unimplemented!()
	/// # 	}
	/// # }
	/// # fn main() {
	/// let ser = TypeMapper::to_string(MyTypeMapping).unwrap();
	/// # }
	/// ```
	pub fn to_string(t: M) -> Result<String, serde_json::Error> {
		let mut writer = Vec::new();
		{
			let mut ser = serde_json::Serializer::new(&mut writer);
			try!(Self::to_writer(t, &mut ser));
		}

		String::from_utf8(writer).map_err(|e| serde_json::Error::custom(e.description()))
	}

	/// Map a user-defined type to a `serde_json::Value`.
	///
	/// The mapping is emitted as a json field, where the key is the name of the type, as defined by `M::data_type()`.
	///
	/// # Examples
	///
	/// ```
	/// # #![feature(plugin, custom_derive)]
	/// # #![plugin(json_str, elastic_types_macros)]
	/// # #[macro_use]
	/// # extern crate elastic_types;
	/// # extern crate serde;
	/// # extern crate serde_json;
	/// # use serde::{ Serialize, Deserialize };
	/// # use elastic_types::mapping::prelude::*;
	/// # use elastic_types::date::prelude::*;
	/// # #[derive(Default, Clone, Serialize, Deserialize, ElasticType)]
	/// # pub struct MyType {
	/// # 	pub my_date: ElasticDate<DefaultDateFormat>,
	/// # 	pub my_string: String,
	/// # 	pub my_num: i32
	/// # }
	/// # impl serde::Serialize for MyType {
	/// # 	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error> where S: serde::Serializer {
	/// # 		unimplemented!()
	/// # 	}
	/// # }
	/// # impl serde::Deserialize for MyType {
	/// # 	 fn deserialize<D>(deserializer: &mut D) -> Result<Self, D::Error> where D: serde::Deserializer {
	/// # 		unimplemented!()
	/// # 	}
	/// # }
	/// # fn main() {
	/// let val = TypeMapper::to_value(MyTypeMapping).unwrap();
	///
	/// let ty = val.lookup("properties.my_date.type");
	/// # }
	/// ```
	pub fn to_value(t: M) -> Result<serde_json::Value, serde_json::Error> {
		let mut ser = serde_json::value::Serializer::new();
		try!(Self::to_writer(t, &mut ser));

		Ok(ser.unwrap())
	}
}