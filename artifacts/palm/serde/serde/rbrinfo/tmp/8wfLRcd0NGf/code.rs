fn serialize_u32(self, v: u32) -> Result<Content, E> {
            Ok(Content::U32(v))
        }