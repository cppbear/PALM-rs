fn visit_i32<F>(self, value: i32) -> Result<Self::Value, F>
        where
            F: de::Error,
        {
            ContentVisitor::new()
                .visit_i32(value)
                .map(TagOrContent::Content)
        }