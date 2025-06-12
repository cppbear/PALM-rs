fn visit_u32<F>(self, value: u32) -> Result<Self::Value, F>
        where
            F: de::Error,
        {
            ContentVisitor::new()
                .visit_u32(value)
                .map(TagOrContent::Content)
        }