use super::models::{DataType, DataTypeFormat, DefaultSchemaRaw, Operation, SchemaRepr};

use std::collections::{BTreeMap, BTreeSet};

/// Interface for the [`Schema`](https://github.com/OAI/OpenAPI-Specification/blob/master/versions/2.0.md#schemaObject) object.
///
/// This is only used for resolving the definitions.
///
/// **NOTE:** Don't implement this by yourself! Please use the `#[api_v2_schema]`
/// proc macro attribute instead.
pub trait Schema: Sized {
    /// Description for this schema, if any (`description` field).
    fn description(&self) -> Option<&str>;

    /// Reference to some other schema, if any (`$ref` field).
    fn reference(&self) -> Option<&str>;

    /// Data type of this schema, if any (`type` field).
    fn data_type(&self) -> Option<DataType>;

    /// Data type format used by this schema, if any (`format` field).
    fn format(&self) -> Option<&DataTypeFormat>;

    /// Schema for array definitions, if any (`items` field).
    fn items(&self) -> Option<&SchemaRepr<Self>>;

    /// Mutable access to the `items` field, if it exists.
    fn items_mut(&mut self) -> Option<&mut SchemaRepr<Self>>;

    /// Value schema for maps (`additional_properties` field).
    fn additional_properties(&self) -> Option<&SchemaRepr<Self>>;

    /// Mutable access to `additional_properties` field, if it's a map.
    fn additional_properties_mut(&mut self) -> Option<&mut SchemaRepr<Self>>;

    /// Map of names and schema for properties, if it's an object (`properties` field)
    fn properties(&self) -> Option<&BTreeMap<String, SchemaRepr<Self>>>;

    /// Mutable access to `properties` field.
    fn properties_mut(&mut self) -> Option<&mut BTreeMap<String, SchemaRepr<Self>>>;

    /// Returns the required properties (if any) for this object.
    fn required_properties(&self) -> Option<&BTreeSet<String>>;

    /// Set whether this definition is cyclic. This is done by the resolver.
    fn set_cyclic(&mut self, cyclic: bool);

    /// Returns whether this definition is cyclic.
    ///
    /// **NOTE:** This is not part of the schema object, but it's
    /// set by the resolver using `set_cyclic` for codegen.
    fn is_cyclic(&self) -> bool;

    /// Name of this schema, if any.
    ///
    /// **NOTE:** This is not part of the schema object, but it's
    /// set by the resolver using `set_name` for codegen.
    fn name(&self) -> Option<&str>;

    /// Sets the name for this schema. This is done by the resolver.
    fn set_name(&mut self, name: &str);
}

/// Trait for returning OpenAPI data type and format for the implementor.
pub trait TypedData {
    /// The OpenAPI type for this implementor.
    fn data_type() -> DataType {
        DataType::Object
    }

    /// The optional OpenAPI data format for this implementor.
    fn format() -> Option<DataTypeFormat> {
        None
    }
}

macro_rules! impl_type_simple {
    ($ty:ty, $dt:expr) => {
        impl TypedData for $ty {
            fn data_type() -> DataType {
                $dt
            }
        }
    };
    ($ty:ty, $dt:expr, $df:expr) => {
        impl TypedData for $ty {
            fn data_type() -> DataType {
                $dt
            }
            fn format() -> Option<DataTypeFormat> {
                Some($df)
            }
        }
    };
}

impl_type_simple!(char, DataType::String);
impl_type_simple!(String, DataType::String);
impl_type_simple!(bool, DataType::Boolean);
impl_type_simple!(f32, DataType::Number, DataTypeFormat::Float);
impl_type_simple!(f64, DataType::Number, DataTypeFormat::Double);
impl_type_simple!(i8, DataType::Integer, DataTypeFormat::Int32);
impl_type_simple!(i16, DataType::Integer, DataTypeFormat::Int32);
impl_type_simple!(i32, DataType::Integer, DataTypeFormat::Int32);
impl_type_simple!(u8, DataType::Integer, DataTypeFormat::Int32);
impl_type_simple!(u16, DataType::Integer, DataTypeFormat::Int32);
impl_type_simple!(u32, DataType::Integer, DataTypeFormat::Int32);
impl_type_simple!(i64, DataType::Integer, DataTypeFormat::Int64);
impl_type_simple!(i128, DataType::Integer, DataTypeFormat::Int64);
impl_type_simple!(isize, DataType::Integer, DataTypeFormat::Int64);
impl_type_simple!(u64, DataType::Integer, DataTypeFormat::Int64);
impl_type_simple!(u128, DataType::Integer, DataTypeFormat::Int64);
impl_type_simple!(usize, DataType::Integer, DataTypeFormat::Int64);

impl<T: TypedData> TypedData for Option<T> {
    fn data_type() -> DataType {
        T::data_type()
    }

    fn format() -> Option<DataTypeFormat> {
        T::format()
    }
}

/// Represents a OpenAPI v2 schema convertible. This is auto-implemented by
/// [`api_v2_schema`](https://paperclip.waffles.space/paperclip_actix_macros/attr.api_v2_schema.html) macro.
///
/// This is implemented for primitive types by default.
pub trait Apiv2Schema {
    /// Name of this schema. This is the object's name.
    const NAME: Option<&'static str>;

    /// Returns the raw schema for this object.
    fn raw_schema() -> DefaultSchemaRaw;

    /// Returns the schema with a reference (if this is an object).
    ///
    /// Here, we set the global reference to this object using its name,
    /// and we either remove the reference (`remove_refs`) or remove all
    /// properties other than the reference (`retain_ref`) based on where
    /// we're storing this object in the spec i.e., in an operation/response
    /// or in the map of definitions.
    ///
    /// And we do that because at the time of this writing, statically
    /// collecting all models for all operations involved a lot of work,
    /// and so I went for runtime collection. Even though this happens at
    /// runtime, we're only doing this once at the start of the application,
    /// so it won't affect the incoming requests at all.
    fn schema_with_ref() -> DefaultSchemaRaw {
        let mut def = Self::raw_schema();
        if let Some(n) = Self::NAME {
            def.reference = Some(String::from("#/definitions/") + n);
        }

        def
    }
}

impl<T: TypedData> Apiv2Schema for T {
    const NAME: Option<&'static str> = None;

    fn raw_schema() -> DefaultSchemaRaw {
        let mut schema = DefaultSchemaRaw::default();
        schema.data_type = Some(T::data_type());
        schema.format = T::format();
        schema
    }
}

macro_rules! impl_schema_array {
    ($ty:ty) => {
        impl<T: Apiv2Schema> Apiv2Schema for $ty {
            const NAME: Option<&'static str> = None;

            fn raw_schema() -> DefaultSchemaRaw {
                let mut schema = DefaultSchemaRaw::default();
                schema.data_type = Some(DataType::Array);
                schema.items = Some(T::schema_with_ref().into());
                schema
            }
        }
    };
}

macro_rules! impl_schema_map {
    ($ty:ty) => {
        impl<K: AsRef<str>, V: Apiv2Schema> Apiv2Schema for $ty {
            const NAME: Option<&'static str> = None;

            fn raw_schema() -> DefaultSchemaRaw {
                let mut schema = DefaultSchemaRaw::default();
                schema.data_type = Some(DataType::Object);
                schema.extra_props = Some(V::schema_with_ref().into());
                schema
            }
        }
    };
}

use std::collections::*;

impl_schema_array!(Vec<T>);
impl_schema_array!(HashSet<T>);
impl_schema_array!(LinkedList<T>);
impl_schema_array!(VecDeque<T>);
impl_schema_array!(BTreeSet<T>);
impl_schema_array!(BinaryHeap<T>);
impl_schema_array!([T; 0]);
impl_schema_array!([T; 1]);
impl_schema_array!([T; 2]);
impl_schema_array!([T; 3]);
impl_schema_array!([T; 4]);
impl_schema_array!([T; 5]);
impl_schema_array!([T; 6]);
impl_schema_array!([T; 7]);
impl_schema_array!([T; 8]);
impl_schema_array!([T; 9]);
impl_schema_array!([T; 10]);
impl_schema_array!([T; 11]);
impl_schema_array!([T; 12]);
impl_schema_array!([T; 13]);
impl_schema_array!([T; 14]);
impl_schema_array!([T; 15]);
impl_schema_array!([T; 16]);

impl_schema_map!(HashMap<K, V>);
impl_schema_map!(BTreeMap<K, V>);

/// Represents a OpenAPI v2 operation convertible. This is auto-implemented by
/// framework-specific macros:
///
/// - [`paperclip_actix::api_v2_operation`](https://paperclip.waffles.space/paperclip_actix_macros/attr.api_v2_operation.html).
pub trait Apiv2Operation {
    /// Returns the definition for this operation.
    fn operation() -> Operation<DefaultSchemaRaw>;

    /// Returns the definitions used by this operation.
    fn definitions() -> BTreeMap<String, DefaultSchemaRaw>;
}
