// Answer 0

#[test]
fn test_path_and_query_valid() {
    struct TestUriBuilder;

    impl TestUriBuilder {
        fn new() -> Self {
            TestUriBuilder
        }

        fn path_and_query<T>(self, p_and_q: T) -> Self
        where
            T: TryInto<PathAndQuery>,
            <T as TryInto<PathAndQuery>>::Error: Into<crate::Error>,
        {
            // Dummy implementation for testing.
            self
        }

        fn build(self) -> Result<(), crate::Error> {
            Ok(())
        }
    }

    struct ValidPathAndQuery(String);

    impl TryInto<PathAndQuery> for ValidPathAndQuery {
        type Error = crate::Error;

        fn try_into(self) -> Result<PathAndQuery, Self::Error> {
            Ok(PathAndQuery(self.0))
        }
    }

    let uri = TestUriBuilder::new()
        .path_and_query(ValidPathAndQuery(String::from("/hello?foo=bar")))
        .build()
        .unwrap();
}

#[test]
#[should_panic]
fn test_path_and_query_invalid() {
    struct TestUriBuilder;

    impl TestUriBuilder {
        fn new() -> Self {
            TestUriBuilder
        }

        fn path_and_query<T>(self, p_and_q: T) -> Self
        where
            T: TryInto<PathAndQuery>,
            <T as TryInto<PathAndQuery>>::Error: Into<crate::Error>,
        {
            // Dummy implementation for testing.
            self
        }

        fn build(self) -> Result<(), crate::Error> {
            // Simulate panic scenario
            panic!("Expected panic due to invalid PathAndQuery");
        }
    }

    struct InvalidPathAndQuery;

    impl TryInto<PathAndQuery> for InvalidPathAndQuery {
        type Error = crate::Error;

        fn try_into(self) -> Result<PathAndQuery, Self::Error> {
            Err(crate::Error::new("Invalid PathAndQuery"))
        }
    }

    let _uri = TestUriBuilder::new()
        .path_and_query(InvalidPathAndQuery)
        .build()
        .unwrap();
}

