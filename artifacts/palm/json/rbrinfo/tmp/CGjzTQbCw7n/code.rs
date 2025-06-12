fn serialize_str(self, value: &str) -> Result<Value> {
        Ok(Value::String(value.to_owned()))
    }