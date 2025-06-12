fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let mut pair_visitor = PairVisitor(Some(self.0), Some(self.1), PhantomData);
        let pair = tri!(visitor.visit_seq(&mut pair_visitor));
        if pair_visitor.1.is_none() {
            Ok(pair)
        } else {
            let remaining = pair_visitor.size_hint().unwrap();
            // First argument is the number of elements in the data, second
            // argument is the number of elements expected by the Deserialize.
            Err(de::Error::invalid_length(2, &ExpectedInSeq(2 - remaining)))
        }
    }