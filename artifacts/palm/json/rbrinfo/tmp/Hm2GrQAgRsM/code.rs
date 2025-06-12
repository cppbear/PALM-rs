fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self)>
    where
        V: de::DeserializeSeed<'de>,
    {
        let val = tri!(seed.deserialize(&mut *self.de));
        tri!(self.de.parse_object_colon());
        Ok((val, self))
    }