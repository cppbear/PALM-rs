fn visit_str<E>(self, field: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            if field == self.tag {
                Ok(TagOrContentField::Tag)
            } else if field == self.content {
                Ok(TagOrContentField::Content)
            } else {
                Err(de::Error::invalid_value(Unexpected::Str(field), &self))
            }
        }