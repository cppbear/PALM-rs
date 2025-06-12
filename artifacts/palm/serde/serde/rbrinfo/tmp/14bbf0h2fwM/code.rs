fn visit_u16<F>(self, value: u16) -> Result<Self::Value, F>
        where
            F: de::Error,
        {
            ContentVisitor::new()
                .visit_u16(value)
                .map(TagOrContent::Content)
        }