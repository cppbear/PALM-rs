fn limit(self, limit: usize) -> Limit<Self>
    where
        Self: Sized,
    {
        limit::new(self, limit)
    }