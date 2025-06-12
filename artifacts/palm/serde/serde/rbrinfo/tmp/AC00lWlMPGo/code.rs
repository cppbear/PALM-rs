fn serialize_value<T>(&mut self, value: &T) -> Result<(), E>
        where
            T: ?Sized + Serialize,
        {
            let key = self
                .key
                .take()
                .expect("serialize_value called before serialize_key");
            let value = tri!(value.serialize(ContentSerializer::<E>::new()));
            self.entries.push((key, value));
            Ok(())
        }