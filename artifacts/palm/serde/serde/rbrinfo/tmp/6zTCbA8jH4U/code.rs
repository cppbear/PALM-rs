fn visit_char<F>(self, value: char) -> Result<Self::Value, F>
        where
            F: de::Error,
        {
            ContentVisitor::new()
                .visit_char(value)
                .map(TagOrContent::Content)
        }