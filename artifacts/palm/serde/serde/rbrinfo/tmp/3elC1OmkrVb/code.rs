fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            match self.content {
                Content::Bool(v) => visitor.visit_bool(v),
                Content::U8(v) => visitor.visit_u8(v),
                Content::U16(v) => visitor.visit_u16(v),
                Content::U32(v) => visitor.visit_u32(v),
                Content::U64(v) => visitor.visit_u64(v),
                Content::I8(v) => visitor.visit_i8(v),
                Content::I16(v) => visitor.visit_i16(v),
                Content::I32(v) => visitor.visit_i32(v),
                Content::I64(v) => visitor.visit_i64(v),
                Content::F32(v) => visitor.visit_f32(v),
                Content::F64(v) => visitor.visit_f64(v),
                Content::Char(v) => visitor.visit_char(v),
                Content::String(v) => visitor.visit_string(v),
                Content::Str(v) => visitor.visit_borrowed_str(v),
                Content::ByteBuf(v) => visitor.visit_byte_buf(v),
                Content::Bytes(v) => visitor.visit_borrowed_bytes(v),
                Content::Unit => visitor.visit_unit(),
                Content::None => visitor.visit_none(),
                Content::Some(v) => visitor.visit_some(ContentDeserializer::new(*v)),
                Content::Newtype(v) => visitor.visit_newtype_struct(ContentDeserializer::new(*v)),
                Content::Seq(v) => visit_content_seq(v, visitor),
                Content::Map(v) => visit_content_map(v, visitor),
            }
        }