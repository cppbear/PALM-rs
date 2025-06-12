fn serialize_char(self, v: char) -> Result<Content, E> {
            Ok(Content::Char(v))
        }