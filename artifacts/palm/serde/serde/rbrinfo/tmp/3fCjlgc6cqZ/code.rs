fn serialize_u16(self, v: u16) -> Result<Content, E> {
            Ok(Content::U16(v))
        }