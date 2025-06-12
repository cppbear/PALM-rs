fn visit_str<F>(self, value: &str) -> Result<Self::Value, F>
        where
            F: de::Error,
        {
            if value == self.name {
                Ok(TagOrContent::Tag)
            } else {
                ContentVisitor::new()
                    .visit_str(value)
                    .map(TagOrContent::Content)
            }
        }