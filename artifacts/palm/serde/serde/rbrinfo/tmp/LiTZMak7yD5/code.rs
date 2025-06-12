fn serialize_bool(self, v: bool) -> Result<Content, E> {
            Ok(Content::Bool(v))
        }