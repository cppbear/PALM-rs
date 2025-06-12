fn serialize_str(self, value: &str) -> Result<Content, E> {
            Ok(Content::String(value.to_owned()))
        }