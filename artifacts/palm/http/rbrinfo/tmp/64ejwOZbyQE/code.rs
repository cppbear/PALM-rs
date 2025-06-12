pub fn extension<T>(self, extension: T) -> Builder
    where
        T: Clone + Any + Send + Sync + 'static,
    {
        self.and_then(move |mut head| {
            head.extensions.insert(extension);
            Ok(head)
        })
    }