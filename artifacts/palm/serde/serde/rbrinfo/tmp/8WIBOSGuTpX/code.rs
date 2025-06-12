fn visit_i16<F>(self, value: i16) -> Result<Self::Value, F>
        where
            F: de::Error,
        {
            ContentVisitor::new()
                .visit_i16(value)
                .map(TagOrContent::Content)
        }