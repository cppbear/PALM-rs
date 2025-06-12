fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        T::Owned::deserialize(deserializer).map(Cow::Owned)
    }