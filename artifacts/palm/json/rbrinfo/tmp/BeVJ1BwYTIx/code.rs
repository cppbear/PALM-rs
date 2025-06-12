fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self)>
    where
        V: de::DeserializeSeed<'de>,
    {
        let variant = tri!(seed.deserialize(&mut *self.de));
        Ok((variant, self))
    }