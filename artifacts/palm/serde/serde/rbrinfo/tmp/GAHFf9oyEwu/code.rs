fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let (start, end) = tri!(deserializer.deserialize_struct(
            "RangeInclusive",
            range::FIELDS,
            range::RangeVisitor {
                expecting: "struct RangeInclusive",
                phantom: PhantomData,
            },
        ));
        Ok(RangeInclusive::new(start, end))
    }