pub fn authority<T>(self, auth: T) -> Self
    where
        T: TryInto<Authority>,
        <T as TryInto<Authority>>::Error: Into<crate::Error>,
    {
        self.map(move |mut parts| {
            let auth = auth.try_into().map_err(Into::into)?;
            parts.authority = Some(auth);
            Ok(parts)
        })
    }