fn end(self) -> Result<Value> {
        Ok(Value::Array(self.vec))
    }