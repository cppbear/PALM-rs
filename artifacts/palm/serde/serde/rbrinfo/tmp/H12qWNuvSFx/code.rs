fn serialize_f64(self, v: f64) -> Result<Content, E> {
            Ok(Content::F64(v))
        }