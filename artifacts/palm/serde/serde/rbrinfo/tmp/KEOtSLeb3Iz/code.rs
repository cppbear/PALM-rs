fn serialize_u8(self, v: u8) -> Result<Content, E> {
            Ok(Content::U8(v))
        }