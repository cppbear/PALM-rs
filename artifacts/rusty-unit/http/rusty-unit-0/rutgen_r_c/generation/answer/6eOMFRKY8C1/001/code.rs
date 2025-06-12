// Answer 0

#[test]
fn test_path_and_query_valid_input() {
    #[derive(Debug)]
    struct ValidPathAndQuery(String);
    impl TryInto<PathAndQuery> for ValidPathAndQuery {
        type Error = crate::Error;
        
        fn try_into(self) -> Result<PathAndQuery, Self::Error> {
            Ok(PathAndQuery(self.0))
        }
    }

    let builder = Builder::new();
    let builder = builder.path_and_query(ValidPathAndQuery(String::from("/test?key=value")));
    let uri_result = builder.build();
    
    assert!(uri_result.is_ok());
}

#[test]
#[should_panic]
fn test_path_and_query_invalid_input() {
    #[derive(Debug)]
    struct InvalidPathAndQuery(String);
    impl TryInto<PathAndQuery> for InvalidPathAndQuery {
        type Error = crate::Error;

        fn try_into(self) -> Result<PathAndQuery, Self::Error> {
            Err(crate::Error { inner: ErrorKind::InvalidUri }) // Simulating an error
        }
    }

    let builder = Builder::new();
    let _ = builder.path_and_query(InvalidPathAndQuery(String::from("/badPath")));
}

#[test]
fn test_path_and_query_empty_input() {
    #[derive(Debug)]
    struct EmptyPathAndQuery(String);
    impl TryInto<PathAndQuery> for EmptyPathAndQuery {
        type Error = crate::Error;

        fn try_into(self) -> Result<PathAndQuery, Self::Error> {
            if self.0.is_empty() {
                return Err(crate::Error { inner: ErrorKind::InvalidUri }); // Simulating an error for empty
            }
            Ok(PathAndQuery(self.0))
        }
    }

    let builder = Builder::new();
    let builder = builder.path_and_query(EmptyPathAndQuery(String::from("")));
    let uri_result = builder.build();

    assert!(uri_result.is_err());
}

