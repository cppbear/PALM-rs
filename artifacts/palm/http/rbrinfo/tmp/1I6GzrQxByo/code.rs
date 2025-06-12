pub fn connect<T>(uri: T) -> Builder
    where
        T: TryInto<Uri>,
        <T as TryInto<Uri>>::Error: Into<crate::Error>,
    {
        Builder::new().method(Method::CONNECT).uri(uri)
    }