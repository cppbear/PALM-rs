fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.map.insert(String::from(key), tri!(to_value(value)));
        Ok(())
    }