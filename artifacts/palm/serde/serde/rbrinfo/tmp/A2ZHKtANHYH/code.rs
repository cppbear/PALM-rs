fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let visitor = PhantomDataVisitor {
            marker: PhantomData,
        };
        deserializer.deserialize_unit_struct("PhantomData", visitor)
    }