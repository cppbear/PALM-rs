fn serialize_entry<K, V>(&mut self, key: &K, value: &V) -> Result<(), E>
        where
            K: ?Sized + Serialize,
            V: ?Sized + Serialize,
        {
            let key = tri!(key.serialize(ContentSerializer::<E>::new()));
            let value = tri!(value.serialize(ContentSerializer::<E>::new()));
            self.entries.push((key, value));
            Ok(())
        }