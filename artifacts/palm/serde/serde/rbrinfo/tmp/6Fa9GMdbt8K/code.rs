fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let capacity = size_hint::cautious::<u8>(seq.size_hint());
        let mut values = Vec::<u8>::with_capacity(capacity);

        while let Some(value) = tri!(seq.next_element()) {
            values.push(value);
        }

        CString::new(values).map_err(Error::custom)
    }