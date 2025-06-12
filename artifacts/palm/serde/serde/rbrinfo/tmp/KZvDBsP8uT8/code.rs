fn next_entry_seed<TK, TV>(
        &mut self,
        kseed: TK,
        vseed: TV,
    ) -> Result<Option<(TK::Value, TV::Value)>, Self::Error>
    where
        TK: de::DeserializeSeed<'de>,
        TV: de::DeserializeSeed<'de>,
    {
        match self.next_pair() {
            Some((key, value)) => {
                let key = tri!(kseed.deserialize(key.into_deserializer()));
                let value = tri!(vseed.deserialize(value.into_deserializer()));
                Ok(Some((key, value)))
            }
            None => Ok(None),
        }
    }