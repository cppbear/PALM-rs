fn serialize_i64(self, value: i64) -> Result<Value> {
        Ok(Value::Number(value.into()))
    }