fn serialize_u64(self, v: u64) -> Result<Content, E> {
            Ok(Content::U64(v))
        }