fn visit_u64<F>(self, value: u64) -> Result<Self::Value, F>
        where
            F: de::Error,
        {
            ContentVisitor::new()
                .visit_u64(value)
                .map(TagOrContent::Content)
        }