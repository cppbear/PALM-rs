pub fn get<T>(uri: T) -> Builder
    where
        T: TryInto<Uri>,
        <T as TryInto<Uri>>::Error: Into<crate::Error>,
    {
        Builder::new().method(Method::GET).uri(uri)
    }