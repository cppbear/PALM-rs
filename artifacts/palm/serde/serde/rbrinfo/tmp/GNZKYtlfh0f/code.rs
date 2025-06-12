fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: SeqAccess<'de>,
        {
            let start: Idx = match tri!(seq.next_element()) {
                Some(value) => value,
                None => {
                    return Err(Error::invalid_length(0, &self));
                }
            };
            Ok(start)
        }