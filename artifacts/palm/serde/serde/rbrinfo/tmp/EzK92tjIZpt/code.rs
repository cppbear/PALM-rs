fn visit_string<F>(self, value: String) -> Result<Self::Value, F>
        where
            F: de::Error,
        {
            if value == self.name {
                Ok(TagOrContent::Tag)
            } else {
                ContentVisitor::new()
                    .visit_string(value)
                    .map(TagOrContent::Content)
            }
        }