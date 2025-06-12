fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        while let Some(IgnoredAny) = tri!(seq.next_element()) {
            // Gobble
        }
        Ok(IgnoredAny)
    }