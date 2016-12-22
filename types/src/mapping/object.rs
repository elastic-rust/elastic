use std::marker::PhantomData;
use serde::{Serialize, Serializer};
use super::field::{FieldType, FieldMapping, Field};

/// Elasticsearch datatype name.
pub const OBJECT_DATATYPE: &'static str = "object";
/// Elasticsearch datatype name.
pub const DYNAMIC_DATATYPE: &'static str = "dynamic";
/// Elasticsearch datatype name.
pub const NESTED_DATATYPE: &'static str = "nested";

/// Serialise a field mapping using the given serialiser.
#[inline]
pub fn field_ser<S, M, F>(serializer: &mut S, state: &mut S::StructState, field: &'static str, _: M) -> Result<(), S::Error>
    where S: Serializer,
          M: FieldMapping<F>,
          F: Default
{
    serializer.serialize_struct_elt(state, field, &M::ser())
}

#[doc(hidden)]
#[derive(Default)]
pub struct ObjectFormat;

/// The base requirements for mapping an `object` type.
pub trait ObjectMapping
    where Self: PropertiesMapping + Default
{
    /// Get the indexed name for this mapping.
    fn name() -> &'static str;

    /// Get the type name for this mapping, like `object` or `nested`.
    fn data_type() -> &'static str {
        NESTED_DATATYPE
    }

    /// Whether or not new properties should be added dynamically to an existing object.
    /// Accepts `true` (default), `false` and `strict`.
    fn dynamic() -> Option<Dynamic> {
        None
    }

    /// Whether the JSON value given for the object field should be parsed and indexed
    /// (`true`, default) or completely ignored (`false`).
    fn enabled() -> Option<bool> {
        None
    }

    /// Sets the default `include_in_all` value for all the properties within the object.
    /// The object itself is not added to the `_all` field.
    fn include_in_all() -> Option<bool> {
        None
    }

    /// Serialise this mapping as an indexed type instead of as a field
    /// on another type.
    /// This excludes all meta properties, like `type` and `enabled` from the json output.
    fn serialize_type<S>(serializer: &mut S) -> Result<(), S::Error>
        where S: Serializer
    {
        let mut state = try!(serializer.serialize_struct("mapping", 1));

        try!(serializer.serialize_struct_elt(&mut state, "properties", &Properties::<Self>::default()));

        serializer.serialize_struct_end(state)
    }
}

/// Serialisation for the mapping of object properties.
///
/// This trait is designed to be auto-derived, so it expects you to be familiar with how `serde` works.
///
/// # Examples
///
/// Say we have a mappable type with 3 fields called `MyType` and a mapping type called `MyTypeMapping`:
///
/// ```
/// # use elastic_types::prelude::*;
/// struct MyType {
/// 	pub my_date: Date<DefaultDateFormat>,
/// 	pub my_string: String,
/// 	pub my_num: i32
/// }
///
/// #[derive(Default)]
/// struct MyTypeMapping;
/// ```
///
/// To serialise the mapping of each of `MyType`s fields, we implement `PropertiesMapping` for `MyTypeMapping`,
/// and use `serde` to serialise the mapping types for each field.
///
/// ```
/// # #![feature(proc_macro)]
/// # #[macro_use]
/// # extern crate json_str;
/// # #[macro_use]
/// # extern crate serde_derive;
/// # #[macro_use]
/// # extern crate elastic_types_derive;
/// # #[macro_use]
/// # extern crate elastic_types;
/// # extern crate serde;
/// # use elastic_types::prelude::*;
/// # pub struct MyTypeMapping;
/// impl PropertiesMapping for MyTypeMapping {
/// 	fn props_len() -> usize { 3 }
///
/// 	fn serialize_props<S>(serializer: &mut S, state: &mut S::StructState) -> Result<(), S::Error>
/// 	where S: serde::Serializer {
/// 		try!(field_ser(serializer, state, "my_date", Date::<DefaultDateFormat>::mapping()));
/// 		try!(field_ser(serializer, state, "my_string", String::mapping()));
/// 		try!(field_ser(serializer, state, "my_num", i32::mapping()));
///
/// 		Ok(())
/// 	}
/// }
/// # fn main() {
/// # }
/// ```
///
/// It's easy to get an instance of the mapping for a given type by calling the static `mapping` function.
/// This trait is automatically implemented for you when you `#[derive(ElasticType)]`.
pub trait PropertiesMapping {
    /// The number of mapped property fields for this type.
    ///
    /// This number should be the same as the number of fields being serialised by `serialize_props`.
    fn props_len() -> usize;

    /// Serialisation for the mapped property fields on this type.
    ///
    /// You can use the `field_ser!` macro to simplify `serde` calls.
    fn serialize_props<S>(serializer: &mut S, state: &mut S::StructState) -> Result<(), S::Error> where S: Serializer;
}

impl<T> FieldMapping<ObjectFormat> for T
    where T: ObjectMapping
{
    type SerType = Field<T, ObjectFormat>;

    fn data_type() -> &'static str {
        <Self as ObjectMapping>::data_type()
    }
}

impl<T> Serialize for Field<T, ObjectFormat>
    where T: FieldMapping<ObjectFormat> + ObjectMapping
{
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: Serializer
    {
        let mut state = try!(serializer.serialize_struct("mapping", 5));

        let ty = <T as ObjectMapping>::data_type();
        try!(serializer.serialize_struct_elt(&mut state, "type", ty));

        ser_field!(serializer, &mut state, "dynamic", T::dynamic());
        ser_field!(serializer,
                   &mut state,
                   "include_in_all",
                   T::include_in_all());

        if ty == OBJECT_DATATYPE {
            ser_field!(serializer, &mut state, "enabled", T::enabled());
        }

        try!(serializer.serialize_struct_elt(&mut state, "properties", &Properties::<T>::default()));

        serializer.serialize_struct_end(state)
    }
}

#[derive(Default)]
struct Properties<M>
    where M: ObjectMapping
{
    _m: PhantomData<M>,
}

impl<M> Serialize for Properties<M>
    where M: ObjectMapping
{
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: Serializer
    {
        let mut state = try!(serializer.serialize_struct("properties", M::props_len()));
        try!(M::serialize_props(serializer, &mut state));
        serializer.serialize_struct_end(state)
    }
}

/// The dynamic setting may be set at the mapping type level, and on each inner object.
/// Inner objects inherit the setting from their parent object or from the mapping type.
#[derive(Debug, Clone, Copy)]
pub enum Dynamic {
    /// Newly detected fields are added to the mapping. (default).
    True,
    /// Newly detected fields are ignored. New fields must be added explicitly.
    False,
    /// If new fields are detected, an exception is thrown and the document is rejected.
    Strict,
}

impl Serialize for Dynamic {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: Serializer
    {
        match *self {
            Dynamic::True => serializer.serialize_bool(true),
            Dynamic::False => serializer.serialize_bool(false),
            Dynamic::Strict => serializer.serialize_str("strict"),
        }
    }
}