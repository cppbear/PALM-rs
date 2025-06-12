fn serialize_bytes(self, value: &[u8]) -> Result<Content, E> {
            Ok(Content::Bytes(value.to_owned()))
        }