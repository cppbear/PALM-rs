// Answer 0

#[test]
fn test_map_with_successful_function() {
    struct TestScheme;
    impl TryInto<Scheme> for TestScheme {
        type Error = crate::Error;

        fn try_into(self) -> Result<Scheme, Self::Error> {
            Ok(Scheme) // Successful conversion for testing
        }
    }

    let builder = Builder { 
        parts: Ok(Parts::default()) 
    };

    let result = builder.map(|parts| {
        let mut new_parts = parts;
        new_parts.scheme = Some(TestScheme.try_into().unwrap());
        Ok(new_parts)
    });

    assert!(result.parts.is_ok());
}

#[test]
fn test_map_with_failure_function() {
    struct FailingScheme;
    impl TryInto<Scheme> for FailingScheme {
        type Error = crate::Error;

        fn try_into(self) -> Result<Scheme, Self::Error> {
            Err(crate::Error { inner: ErrorKind }) // Simulating failure
        }
    }

    let builder = Builder { 
        parts: Ok(Parts::default()) 
    };

    let result = builder.map(|parts| {
        let mut new_parts = parts;
        new_parts.scheme = Some(FailingScheme.try_into().unwrap_err());
        Ok(new_parts)
    });

    assert!(result.parts.is_err());
}

#[test]
fn test_map_on_empty_parts() {
    let builder = Builder { 
        parts: Err(crate::Error { inner: ErrorKind }) 
    };

    let result = builder.map(|parts| Ok(parts)); // Function that returns the parts without modification

    assert!(result.parts.is_err());
}

