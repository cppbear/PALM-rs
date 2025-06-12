fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), E>
        where
            T: ?Sized + Serialize,
        {
            let value = tri!(value.serialize(ContentSerializer::<E>::new()));
            self.fields.push((key, value));
            Ok(())
        }