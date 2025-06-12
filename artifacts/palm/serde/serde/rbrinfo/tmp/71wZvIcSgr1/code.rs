fn struct_variant<V>(
            mut self,
            _fields: &'static [&'static str],
            visitor: V,
        ) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            self.map.next_value_seed(SeedStructVariant { visitor })
        }