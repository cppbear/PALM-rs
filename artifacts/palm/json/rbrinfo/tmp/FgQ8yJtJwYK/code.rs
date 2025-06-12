fn serialize_f64(self, float: f64) -> Result<Value> {
        Ok(Value::from(float))
    }