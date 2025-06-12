fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let end = tri!(deserializer.deserialize_struct(
            "RangeTo",
            range_to::FIELDS,
            range_to::RangeToVisitor {
                expecting: "struct RangeTo",
                phantom: PhantomData,
            },
        ));
        Ok(..end)
    }