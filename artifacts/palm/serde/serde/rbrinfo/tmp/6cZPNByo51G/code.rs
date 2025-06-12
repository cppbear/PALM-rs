fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match visitor.__private_visit_untagged_option(self) {
            Ok(value) => Ok(value),
            Err(()) => Self::deserialize_other(),
        }
    }