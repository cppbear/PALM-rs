fn end(self) -> Result<Value> {
        let mut object = Map::new();

        object.insert(self.name, Value::Object(self.map));

        Ok(Value::Object(object))
    }