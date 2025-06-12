fn serialize_field<T>(&mut self, value: &T) -> Result<(), E>
        where
            T: ?Sized + Serialize,
        {
            let value = tri!(value.serialize(ContentSerializer::<E>::new()));
            self.fields.push(value);
            Ok(())
        }