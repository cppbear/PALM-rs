fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            match self.content {
                Content::String(v) => visitor.visit_string(v),
                Content::Str(v) => visitor.visit_borrowed_str(v),
                Content::ByteBuf(v) => visitor.visit_byte_buf(v),
                Content::Bytes(v) => visitor.visit_borrowed_bytes(v),
                Content::U8(v) => visitor.visit_u8(v),
                Content::U64(v) => visitor.visit_u64(v),
                _ => Err(self.invalid_type(&visitor)),
            }
        }