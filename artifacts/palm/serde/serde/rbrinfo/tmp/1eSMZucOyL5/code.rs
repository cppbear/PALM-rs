fn deserialize<D>(deserializer: D) -> Result<IgnoredAny, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_ignored_any(IgnoredAny)
    }