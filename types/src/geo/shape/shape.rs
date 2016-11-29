use std::marker::PhantomData;
use serde::{Serialize, Deserialize, Serializer, Deserializer};
use geojson::Geometry;
use super::mapping::*;
use ::mapping::ElasticType;

/// Geo shape type with a given mapping.
///
/// Defining a `geo_shape` with a mapping:
///
/// ```
/// # extern crate elastic_types;
/// extern crate geojson;
/// use geojson::{ Geometry, Value };
///
/// # use elastic_types::prelude::*;
/// # fn main() {
/// let point: GeoShape<DefaultGeoShapeMapping> = GeoShape::new(
///     Geometry::new(
///         Value::Point(vec![ 1.0, 1.0 ])
///     )
/// );
/// # }
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct GeoShape<M = DefaultGeoShapeMapping>
    where M: GeoShapeMapping
{
    value: Geometry,
    _m: PhantomData<M>,
}

impl<M> GeoShape<M>
    where M: GeoShapeMapping
{
    /// Creates a new `GeoShape` from the given `Geometry`.
    ///
    /// This function will consume the provided `Geometry`.
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate elastic_types;
    /// # extern crate geojson;
    /// use geojson::{ Geometry, Value };
    ///
    /// # use elastic_types::prelude::*;
    /// # fn main() {
    /// let point: GeoShape = GeoShape::new(
    ///     Geometry::new(
    ///         Value::Point(vec![ 1.0, 1.0 ])
    ///     )
    /// );
    /// # }
    /// ```
    pub fn new<I>(geo: I) -> GeoShape<M>
        where I: Into<Geometry>
    {
        GeoShape {
            value: geo.into(),
            _m: PhantomData,
        }
    }

    /// Change the mapping of this geo shape.
    pub fn remap<MInto: GeoShapeMapping>(self) -> GeoShape<MInto> {
        GeoShape::<MInto>::new(self.value)
    }
}

impl<M> ElasticType<M, GeoShapeFormat> for GeoShape<M> where M: GeoShapeMapping {}

impl_mapping_type!(Geometry, GeoShape, GeoShapeMapping);

impl<M> Serialize for GeoShape<M>
    where M: GeoShapeMapping
{
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: Serializer
    {
        self.value.serialize(serializer)
    }
}

impl<M> Deserialize for GeoShape<M>
    where M: GeoShapeMapping
{
    fn deserialize<D>(deserializer: &mut D) -> Result<GeoShape<M>, D::Error>
        where D: Deserializer
    {
        let t = try!(Geometry::deserialize(deserializer));

        Ok(GeoShape::<M>::new(t))
    }
}
