fn visit_u64<E>(self, field_index: u64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            match field_index {
                0 => Ok(TagOrContentField::Tag),
                1 => Ok(TagOrContentField::Content),
                _ => Err(de::Error::invalid_value(
                    Unexpected::Unsigned(field_index),
                    &self,
                )),
            }
        }