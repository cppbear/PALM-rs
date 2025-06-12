pub fn status<T>(self, status: T) -> Builder
    where
        T: TryInto<StatusCode>,
        <T as TryInto<StatusCode>>::Error: Into<crate::Error>,
    {
        self.and_then(move |mut head| {
            head.status = status.try_into().map_err(Into::into)?;
            Ok(head)
        })
    }