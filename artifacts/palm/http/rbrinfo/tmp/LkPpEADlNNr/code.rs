pub fn uri<T>(self, uri: T) -> Builder
    where
        T: TryInto<Uri>,
        <T as TryInto<Uri>>::Error: Into<crate::Error>,
    {
        self.and_then(move |mut head| {
            head.uri = uri.try_into().map_err(Into::into)?;
            Ok(head)
        })
    }