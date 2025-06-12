fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let (start, end) = tri!(deserializer.deserialize_struct(
            "Range",
            range::FIELDS,
            range::RangeVisitor {
                expecting: "struct Range",
                phantom: PhantomData,
            },
        ));
        Ok(start..end)
    }