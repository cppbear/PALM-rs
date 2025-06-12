fn end(self) -> Result<Value> {
        let mut object = Map::new();

        object.insert(self.name, Value::Array(self.vec));

        Ok(Value::Object(object))
    }