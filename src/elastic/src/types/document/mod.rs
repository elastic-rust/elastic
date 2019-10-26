/*!
Base requirements for indexable document mappings.

Structures that can be indexed in Elasticsearch should implement `DocumentType`.
The `DocumentType` is composed of typical mapping metadata, as well as the mapping
for each of its properties.

Documents can be mapped as indexable types, or as an object field on another type.

# Examples

Define your Elasticsearch types using _Plain Old Rust Structures_.

## Derive Mapping

Mapping can be generated by deriving `ElasticType` on a struct:

```
# #[macro_use] extern crate serde_derive;
# #[macro_use] extern crate elastic_derive;
# #[macro_use] use elastic::types::prelude::*;
#[derive(Serialize, ElasticType)]
pub struct MyType {
    pub my_date: Date<DefaultDateMapping>,
    pub my_string: String,
    pub my_num: i32
}
```

This will produce the following field mapping:

```
# #[macro_use] extern crate serde_derive;
# #[macro_use] extern crate elastic_derive;
# #[macro_use] extern crate serde_json;
# use elastic::types::prelude::*;
# #[derive(Serialize, ElasticType)]
# pub struct MyType {
#   pub my_date: Date<DefaultDateMapping>,
#   pub my_string: String,
#   pub my_num: i32
# }
# let mapping = elastic::types::__derive::standalone_field_ser(MyTypeMapping).unwrap();
# let json = json!(
{
    "type": "nested",
    "properties": {
        "my_date": {
            "type": "date",
            "format": "basic_date_time"
        },
        "my_string": {
            "type": "text",
            "fields": {
                "keyword":{
                    "type":"keyword",
                    "ignore_above":256
                }
            }
        },
        "my_num": {
            "type": "integer"
        }
    }
}
# );
# assert_eq!(json, mapping);
```

It's also possible to adjust the mapping using the `#[elastic]` attribute.

### Specifying a default index name

Documents will default to using an index name that's derived from the name of the Rust type.
The `#[elastic(index)]` attribute can be used to set the index name that documents belong to:

```
# #[macro_use] extern crate serde_derive;
# #[macro_use] extern crate elastic_derive;
# #[macro_use] use elastic::types::prelude::*;
#[derive(Serialize, ElasticType)]
#[elastic(index = "my-index")]
pub struct MyType {
    pub my_date: Date<DefaultDateMapping>,
    pub my_string: String,
    pub my_num: i32
}
```

Not all documents have a static index that's the same for all instances though.
For index names that depend on document fields, use the `#[elastic(index(expr = "function"))]` attribute:

```
# #[macro_use] extern crate serde_derive;
# #[macro_use] extern crate elastic_derive;
# #[macro_use] use elastic::types::prelude::*;
#[derive(Serialize, ElasticType)]
#[elastic(index(expr = "self.index()"))]
pub struct MyType {
    pub my_date: Date<DefaultDateMapping>,
    pub my_string: String,
    pub my_num: i32
}

impl MyType {
    fn index(&self) -> String {
        format!("my-index-{}", self.my_date)
    }
}
```

### Specifying a type name

Documents will default to using `_doc` as the type name.
The `#[elastic(ty)]` attribute can be used to set the type name that documents belong to:

```
# #[macro_use] extern crate serde_derive;
# #[macro_use] extern crate elastic_derive;
# #[macro_use] use elastic::types::prelude::*;
#[derive(Serialize, ElasticType)]
#[elastic(ty = "my-type")]
pub struct MyType {
    pub my_date: Date<DefaultDateMapping>,
    pub my_string: String,
    pub my_num: i32
}
```

### Specifying an id field

Documents will default to not using a field as an id.
The `#[elastic(id)]` attribute can be used to specify an id field:

```
# #[macro_use] extern crate serde_derive;
# #[macro_use] extern crate elastic_derive;
# #[macro_use] use elastic::types::prelude::*;
#[derive(Serialize, ElasticType)]
pub struct MyType {
    #[elastic(id)]
    pub my_id: String,
    pub my_date: Date<DefaultDateMapping>,
    pub my_string: String,
    pub my_num: i32
}
```

The field annotated with `#[elastic(id)]` must satisfy `impl Into<Cow<'_, str>>`.
An id can also be calculated based on an expression function using the `#[elastic(id(expr = "expression"))]` attribute:

```
# #[macro_use] extern crate serde_derive;
# #[macro_use] extern crate elastic_derive;
# #[macro_use] use elastic::types::prelude::*;
#[derive(Serialize, ElasticType)]
#[elastic(id(expr = "self.id()"))]
pub struct MyType {
    pub my_id: i32,
    pub my_date: Date<DefaultDateMapping>,
    pub my_string: String,
    pub my_num: i32
}

impl MyType {
    fn id(&self) -> String {
        self.my_id.to_string()
    }
}
```

An expression can also be used on fields, where an identifier with the same name as the field can be used:

```
# #[macro_use] extern crate serde_derive;
# #[macro_use] extern crate elastic_derive;
# #[macro_use] use elastic::types::prelude::*;
#[derive(Serialize, ElasticType)]
pub struct MyType {
    #[elastic(id(expr = "my_id.to_string()"))]
    pub my_id: i32,
    pub my_date: Date<DefaultDateMapping>,
    pub my_string: String,
    pub my_num: i32
}
```

### Override Default Mapping Properties

You can override the mapping meta properties for an object by providing your own mapping type with `#[elastic(mapping="{TypeName}")]`:

```
# #[macro_use] extern crate serde_derive;
# #[macro_use] extern crate elastic_derive;
# #[macro_use] use elastic::types::prelude::*;
#[derive(Serialize, ElasticType)]
#[elastic(mapping="MyTypeMapping")]
pub struct MyType {
    pub my_date: Date<DefaultDateMapping>,
    pub my_string: String,
    pub my_num: i32
}

#[derive(Default)]
pub struct MyTypeMapping;
impl ObjectMapping for MyTypeMapping {
    type Properties = MyType;

    fn data_type() -> &'static str { OBJECT_DATATYPE }
}
```

This will produce the following field mapping:

```
# #[macro_use] extern crate serde_derive;
# #[macro_use] extern crate elastic_derive;
# #[macro_use] extern crate serde_json;
# use elastic::types::prelude::*;
# #[derive(Default, Serialize, Deserialize, ElasticType)]
# #[elastic(mapping="MyTypeMapping")]
# pub struct MyType {
#   pub my_date: Date<DefaultDateMapping>,
#   pub my_string: String,
#   pub my_num: i32
# }
#
# #[derive(Default)]
# pub struct MyTypeMapping;
# impl ObjectMapping for MyTypeMapping {
#   type Properties = MyType;
#   fn data_type() -> &'static str { OBJECT_DATATYPE }
# }
# let mapping = elastic::types::__derive::standalone_field_ser(MyTypeMapping).unwrap();
# let json = json!(
{
    "type": "object",
    "properties": {
        "my_date": {
            "type": "date",
            "format": "basic_date_time"
        },
        "my_string": {
            "type": "text",
            "fields": {
                "keyword":{
                    "type":"keyword",
                    "ignore_above":256
                }
            }
        },
        "my_num": {
            "type": "integer"
        }
    }
}
# );
# assert_eq!(json, mapping);
```

### Ignore or Rename Fields

You can then serialise type mappings with `#[serde]` attributes:

```
# #[macro_use] extern crate serde_derive;
# #[macro_use] extern crate elastic_derive;
# #[macro_use] use elastic::types::prelude::*;
#[derive(ElasticType, Serialize)]
pub struct MyType {
    #[serde(rename="my_renamed_date")]
    pub my_date: Date<DefaultDateMapping>,
    #[serde(skip_serializing)]
    pub ignored: String,
    pub my_num: i32
}
```

> NOTE: Fields with a `#[serde(skip_deserializing)]` attribute will still be mapped, because they can
still be indexed in Elasticsearch.

## Limitations

Automatically deriving mapping has the following limitations:

- Generics aren't supported by auto deriving.
So you can't `#[derive(ElasticType)]` on `MyType<T>`.
- Mapping types can't be shared. This is because they need to map the type fields, so are specific to that type.
So you can't share `MyTypeMapping` between `MyType` and `MyOtherType`.

# Links
- [Field Types](https://www.elastic.co/guide/en/elasticsearch/reference/current/mapping-types.html)
- [Document Types](https://www.elastic.co/guide/en/elasticsearch/reference/current/mapping.html)
*/

pub mod mapping;

mod impls;
pub use self::impls::*;

pub mod prelude {
    /*!
    Includes all types for document types.

    This is a convenience module to make it easy to build mappings for multiple types without too many `use` statements.
    */

    pub use super::{
        impls::{
            DocumentType,
            IndexDocumentMapping,
            StaticIndex,
            StaticType,
        },
        mapping::*,
    };
}
