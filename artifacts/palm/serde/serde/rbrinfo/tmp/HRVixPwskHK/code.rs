fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        if len == 2 {
            self.deserialize_seq(visitor)
        } else {
            // First argument is the number of elements in the data, second
            // argument is the number of elements expected by the Deserialize.
            Err(de::Error::invalid_length(2, &ExpectedInSeq(len)))
        }
    }