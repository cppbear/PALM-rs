// Answer 0

#[test]
fn test_header_with_valid_key_value() {
    use crate::header::{HeaderMap, HeaderName, HeaderValue};
    use crate::method::Method;
    use crate::version::Version;
    use std::convert::TryInto;

    #[derive(Default)]
    struct TestBuilder {
        headers: HeaderMap<HeaderValue>,
    }

    impl TestBuilder {
        fn new() -> Self {
            Self::default()
        }

        fn header<K, V>(mut self, key: K, value: V) -> Self
        where
            K: TryInto<HeaderName>,
            <K as TryInto<HeaderName>>::Error: Into<crate::Error>,
            V: TryInto<HeaderValue>,
            <V as TryInto<HeaderValue>>::Error: Into<crate::Error>,
        {
            let name = key.try_into().map_err(Into::into).unwrap();
            let value = value.try_into().map_err(Into::into).unwrap();
            self.headers.try_append(name, value).unwrap();
            self
        }
    }

    let builder = TestBuilder::new();
    let updated_builder = builder.header("Accept", "text/html");
    assert!(updated_builder.headers.get("Accept").is_some());
}

#[test]
#[should_panic]
fn test_header_with_invalid_key() {
    use crate::header::{HeaderName, HeaderValue};
        
    #[derive(Default)]
    struct TestBuilder {
        headers: HeaderMap<HeaderValue>,
    }

    impl TestBuilder {
        fn new() -> Self {
            Self::default()
        }

        fn header<K, V>(mut self, key: K, value: V) -> Self
        where
            K: TryInto<HeaderName>,
            <K as TryInto<HeaderName>>::Error: Into<crate::Error>,
            V: TryInto<HeaderValue>,
            <V as TryInto<HeaderValue>>::Error: Into<crate::Error>,
        {
            let name = key.try_into().map_err(Into::into).unwrap();
            let value = value.try_into().map_err(Into::into).unwrap();
            self.headers.try_append(name, value).unwrap();
            self
        }
    }

    let builder = TestBuilder::new();
    let _: TestBuilder = builder.header("\0", "value"); // Invalid key (null byte)
}

#[test]
fn test_header_with_multiple_entries() {
    use crate::header::{HeaderMap, HeaderName, HeaderValue};
        
    #[derive(Default)]
    struct TestBuilder {
        headers: HeaderMap<HeaderValue>,
    }

    impl TestBuilder {
        fn new() -> Self {
            Self::default()
        }

        fn header<K, V>(mut self, key: K, value: V) -> Self
        where
            K: TryInto<HeaderName>,
            <K as TryInto<HeaderName>>::Error: Into<crate::Error>,
            V: TryInto<HeaderValue>,
            <V as TryInto<HeaderValue>>::Error: Into<crate::Error>,
        {
            let name = key.try_into().map_err(Into::into).unwrap();
            let value = value.try_into().map_err(Into::into).unwrap();
            self.headers.try_append(name, value).unwrap();
            self
        }
    }

    let builder = TestBuilder::new();
    let updated_builder = builder.header("Accept", "text/html").header("X-Custom-Foo", "bar");
    assert!(updated_builder.headers.get("Accept").is_some());
    assert!(updated_builder.headers.get("X-Custom-Foo").is_some());
}

#[test]
#[should_panic]
fn test_header_with_invalid_value() {
    use crate::header::{HeaderName, HeaderValue};
        
    #[derive(Default)]
    struct TestBuilder {
        headers: HeaderMap<HeaderValue>,
    }

    impl TestBuilder {
        fn new() -> Self {
            Self::default()
        }

        fn header<K, V>(mut self, key: K, value: V) -> Self
        where
            K: TryInto<HeaderName>,
            <K as TryInto<HeaderName>>::Error: Into<crate::Error>,
            V: TryInto<HeaderValue>,
            <V as TryInto<HeaderValue>>::Error: Into<crate::Error>,
        {
            let name = key.try_into().map_err(Into::into).unwrap();
            let value = value.try_into().map_err(Into::into).unwrap();
            self.headers.try_append(name, value).unwrap();
            self
        }
    }

    let builder = TestBuilder::new();
    let _: TestBuilder = builder.header("Header-Name", "\0"); // Invalid value (null byte)
}

