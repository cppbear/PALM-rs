fn serialize_u64(self, value: u64) -> Result<Value> {
        Ok(Value::Number(value.into()))
    }