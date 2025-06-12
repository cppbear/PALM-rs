pub fn post<T>(uri: T) -> Builder
    where
        T: TryInto<Uri>,
        <T as TryInto<Uri>>::Error: Into<crate::Error>,
    {
        Builder::new().method(Method::POST).uri(uri)
    }