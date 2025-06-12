fn serialize_i32(self, v: i32) -> Result<Content, E> {
            Ok(Content::I32(v))
        }