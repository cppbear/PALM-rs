fn serialize_i64(self, v: i64) -> Result<Content, E> {
            Ok(Content::I64(v))
        }