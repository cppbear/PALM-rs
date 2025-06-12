fn serialize_bool(self, value: bool) -> Result<Value> {
        Ok(Value::Bool(value))
    }