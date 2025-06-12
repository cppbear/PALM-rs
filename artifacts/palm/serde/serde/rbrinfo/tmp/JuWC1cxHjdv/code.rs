fn visit_f32<F>(self, value: f32) -> Result<Self::Value, F>
        where
            F: de::Error,
        {
            ContentVisitor::new()
                .visit_f32(value)
                .map(TagOrContent::Content)
        }