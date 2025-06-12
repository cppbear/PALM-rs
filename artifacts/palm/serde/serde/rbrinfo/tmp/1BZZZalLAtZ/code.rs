fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let start = tri!(deserializer.deserialize_struct(
            "RangeFrom",
            range_from::FIELDS,
            range_from::RangeFromVisitor {
                expecting: "struct RangeFrom",
                phantom: PhantomData,
            },
        ));
        Ok(start..)
    }