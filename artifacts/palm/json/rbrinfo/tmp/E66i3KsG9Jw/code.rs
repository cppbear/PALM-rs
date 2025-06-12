fn serialize_bytes(self, value: &[u8]) -> Result<Value> {
        let vec = value.iter().map(|&b| Value::Number(b.into())).collect();
        Ok(Value::Array(vec))
    }