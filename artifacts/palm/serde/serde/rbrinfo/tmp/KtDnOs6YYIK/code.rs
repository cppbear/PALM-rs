fn visit_u64<E>(self, field_index: u64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            match field_index {
                0 => Ok(TagContentOtherField::Tag),
                1 => Ok(TagContentOtherField::Content),
                _ => Ok(TagContentOtherField::Other),
            }
        }