fn serialize_key<T>(&mut self, key: &T) -> Result<(), E>
        where
            T: ?Sized + Serialize,
        {
            let key = tri!(key.serialize(ContentSerializer::<E>::new()));
            self.key = Some(key);
            Ok(())
        }