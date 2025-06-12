pub fn header<K, V>(self, key: K, value: V) -> Builder
    where
        K: TryInto<HeaderName>,
        <K as TryInto<HeaderName>>::Error: Into<crate::Error>,
        V: TryInto<HeaderValue>,
        <V as TryInto<HeaderValue>>::Error: Into<crate::Error>,
    {
        self.and_then(move |mut head| {
            let name = key.try_into().map_err(Into::into)?;
            let value = value.try_into().map_err(Into::into)?;
            head.headers.try_append(name, value)?;
            Ok(head)
        })
    }