fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        while let Some((IgnoredAny, IgnoredAny)) = tri!(map.next_entry()) {
            // Gobble
        }
        Ok(IgnoredAny)
    }