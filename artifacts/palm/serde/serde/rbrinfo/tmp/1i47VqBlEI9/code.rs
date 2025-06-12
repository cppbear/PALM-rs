fn deserialize_float<V>(self, visitor: V) -> Result<V::Value, E>
        where
            V: Visitor<'de>,
        {
            match self.content {
                Content::F32(v) => visitor.visit_f32(v),
                Content::F64(v) => visitor.visit_f64(v),
                Content::U8(v) => visitor.visit_u8(v),
                Content::U16(v) => visitor.visit_u16(v),
                Content::U32(v) => visitor.visit_u32(v),
                Content::U64(v) => visitor.visit_u64(v),
                Content::I8(v) => visitor.visit_i8(v),
                Content::I16(v) => visitor.visit_i16(v),
                Content::I32(v) => visitor.visit_i32(v),
                Content::I64(v) => visitor.visit_i64(v),
                _ => Err(self.invalid_type(&visitor)),
            }
        }