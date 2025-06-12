fn tuple_variant<V>(mut self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            self.map.next_value_seed(SeedTupleVariant { len, visitor })
        }