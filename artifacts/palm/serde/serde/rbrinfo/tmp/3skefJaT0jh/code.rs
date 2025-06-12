fn visit_i64<F>(self, value: i64) -> Result<Self::Value, F>
        where
            F: de::Error,
        {
            ContentVisitor::new()
                .visit_i64(value)
                .map(TagOrContent::Content)
        }