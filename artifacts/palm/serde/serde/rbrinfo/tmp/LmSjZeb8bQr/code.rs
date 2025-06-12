fn visit_u8<F>(self, value: u8) -> Result<Self::Value, F>
        where
            F: de::Error,
        {
            ContentVisitor::new()
                .visit_u8(value)
                .map(TagOrContent::Content)
        }