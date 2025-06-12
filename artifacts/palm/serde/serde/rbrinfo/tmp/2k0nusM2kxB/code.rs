fn visit_i8<F>(self, value: i8) -> Result<Self::Value, F>
        where
            F: de::Error,
        {
            ContentVisitor::new()
                .visit_i8(value)
                .map(TagOrContent::Content)
        }