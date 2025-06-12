pub fn method<T>(self, method: T) -> Builder
    where
        T: TryInto<Method>,
        <T as TryInto<Method>>::Error: Into<crate::Error>,
    {
        self.and_then(move |mut head| {
            let method = method.try_into().map_err(Into::into)?;
            head.method = method;
            Ok(head)
        })
    }