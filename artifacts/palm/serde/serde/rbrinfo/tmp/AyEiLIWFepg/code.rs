fn serialize_f32(self, v: f32) -> Result<Content, E> {
            Ok(Content::F32(v))
        }