fn visit_f64<F>(self, value: f64) -> Result<Self::Value, F>
        where
            F: de::Error,
        {
            ContentVisitor::new()
                .visit_f64(value)
                .map(TagOrContent::Content)
        }