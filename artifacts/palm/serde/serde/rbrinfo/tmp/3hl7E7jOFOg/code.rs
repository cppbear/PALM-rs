fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            match self.content {
                Content::String(v) => visitor.visit_string(v),
                Content::Str(v) => visitor.visit_borrowed_str(v),
                Content::ByteBuf(v) => visitor.visit_byte_buf(v),
                Content::Bytes(v) => visitor.visit_borrowed_bytes(v),
                Content::Seq(v) => visit_content_seq(v, visitor),
                _ => Err(self.invalid_type(&visitor)),
            }
        }