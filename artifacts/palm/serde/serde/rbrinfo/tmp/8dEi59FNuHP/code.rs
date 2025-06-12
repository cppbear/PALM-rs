fn visit_seq<A>(self, _: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        Ok([])
    }