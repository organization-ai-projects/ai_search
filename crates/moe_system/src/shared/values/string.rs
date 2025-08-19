use crate::shared::{ToValue, Value};

#[derive(Clone, Debug)]
pub struct String {
    pub data: std::string::String,
}

impl ToValue for String {
    fn to_value(self) -> Value {
        Value::Text {
            schema: 1,
            data: self.data,
        }
    }
}
