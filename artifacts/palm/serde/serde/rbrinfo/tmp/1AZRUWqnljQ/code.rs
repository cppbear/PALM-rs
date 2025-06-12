fn visit_bytes<E>(self, field: &[u8]) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            if field == self.tag.as_bytes() {
                Ok(TagOrContentField::Tag)
            } else if field == self.content.as_bytes() {
                Ok(TagOrContentField::Content)
            } else {
                Err(de::Error::invalid_value(Unexpected::Bytes(field), &self))
            }
        }