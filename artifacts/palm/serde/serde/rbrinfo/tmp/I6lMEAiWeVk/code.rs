fn visit_borrowed_str<F>(self, value: &'de str) -> Result<Self::Value, F>
        where
            F: de::Error,
        {
            if value == self.name {
                Ok(TagOrContent::Tag)
            } else {
                ContentVisitor::new()
                    .visit_borrowed_str(value)
                    .map(TagOrContent::Content)
            }
        }