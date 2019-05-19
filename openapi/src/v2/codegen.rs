use super::{
    models::{DataType, DataTypeFormat},
    Schema,
};

pub(crate) trait SchemaExt: Schema {
    fn rust_unit_type_str(&self) -> Option<&'static str> {
        return match self.format() {
            Some(DataTypeFormat::Int32) => Some("i32"),
            Some(DataTypeFormat::Int64) => Some("i64"),
            Some(DataTypeFormat::Float) => Some("f32"),
            Some(DataTypeFormat::Double) => Some("f64"),
            _ => match self.data_type() {
                Some(DataType::Integer) => Some("i64"),
                Some(DataType::Number) => Some("f64"),
                Some(DataType::Boolean) => Some("bool"),
                Some(DataType::String) => Some("String"),
                _ => None,
            },
        };
    }
}
