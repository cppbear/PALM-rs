fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: Deserializer<'de>,
        {
            // Internally tagged enums are only supported in self-describing
            // formats.
            deserializer.deserialize_any(self)
        }