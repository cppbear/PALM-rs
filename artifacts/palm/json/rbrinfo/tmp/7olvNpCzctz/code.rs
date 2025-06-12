fn serialize_f32(self, float: f32) -> Result<Value> {
        Ok(Value::from(float))
    }