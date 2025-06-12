fn index_or_insert<'v>(&self, v: &'v mut Value) -> &'v mut Value {
        if let Value::Null = v {
            *v = Value::Object(Map::new());
        }
        match v {
            Value::Object(map) => map.entry(self.to_owned()).or_insert(Value::Null),
            _ => panic!("cannot access key {:?} in JSON {}", self, Type(v)),
        }
    }