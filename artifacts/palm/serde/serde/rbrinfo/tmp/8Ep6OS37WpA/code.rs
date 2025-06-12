fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        let value = tri!(value.serialize(ContentSerializer::<M::Error>::new()));
        self.fields.push((key, value));
        Ok(())
    }