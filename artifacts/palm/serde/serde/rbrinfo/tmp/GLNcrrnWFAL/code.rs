fn visit_byte_buf<F>(self, value: Vec<u8>) -> Result<Self::Value, F>
        where
            F: de::Error,
        {
            if value == self.name.as_bytes() {
                Ok(TagOrContent::Tag)
            } else {
                ContentVisitor::new()
                    .visit_byte_buf(value)
                    .map(TagOrContent::Content)
            }
        }