// Answer 0

#[test]
fn test_map_with_string_body() {
    struct TestHeaderMap;
    struct TestExtensions;

    impl Default for TestHeaderMap {
        fn default() -> Self {
            TestHeaderMap
        }
    }

    impl Default for TestExtensions {
        fn default() -> Self {
            TestExtensions
        }
    }

    struct TestParts {
        status: StatusCode,
        version: Version,
        headers: TestHeaderMap,
        extensions: TestExtensions,
    }

    impl TestParts {
        fn new() -> Self {
            TestParts {
                status: StatusCode::OK,
                version: Version::HTTP_11,
                headers: TestHeaderMap::default(),
                extensions: TestExtensions::default(),
            }
        }
    }

    let parts = TestParts::new();
    let response: Response<String> = Response::from_parts(parts, String::from("some string"));
    
    let mapped_response: Response<Vec<u8>> = response.map(|b| {
        assert_eq!(b, "some string");
        b.as_bytes().to_vec()
    });
    
    assert_eq!(mapped_response.body(), &b"some string".to_vec());
}

#[test]
fn test_map_with_integer_body() {
    struct TestHeaderMap;
    struct TestExtensions;

    impl Default for TestHeaderMap {
        fn default() -> Self {
            TestHeaderMap
        }
    }

    impl Default for TestExtensions {
        fn default() -> Self {
            TestExtensions
        }
    }

    struct TestParts {
        status: StatusCode,
        version: Version,
        headers: TestHeaderMap,
        extensions: TestExtensions,
    }

    impl TestParts {
        fn new() -> Self {
            TestParts {
                status: StatusCode::OK,
                version: Version::HTTP_11,
                headers: TestHeaderMap::default(),
                extensions: TestExtensions::default(),
            }
        }
    }

    let parts = TestParts::new();
    let response: Response<i32> = Response::from_parts(parts, 42);
    
    let mapped_response: Response<String> = response.map(|b| {
        assert_eq!(b, 42);
        b.to_string()
    });
    
    assert_eq!(mapped_response.body(), "42");
}

#[test]
#[should_panic]
fn test_map_with_panic() {
    struct TestHeaderMap;
    struct TestExtensions;

    impl Default for TestHeaderMap {
        fn default() -> Self {
            TestHeaderMap
        }
    }

    impl Default for TestExtensions {
        fn default() -> Self {
            TestExtensions
        }
    }

    struct TestParts {
        status: StatusCode,
        version: Version,
        headers: TestHeaderMap,
        extensions: TestExtensions,
    }

    impl TestParts {
        fn new() -> Self {
            TestParts {
                status: StatusCode::OK,
                version: Version::HTTP_11,
                headers: TestHeaderMap::default(),
                extensions: TestExtensions::default(),
            }
        }
    }

    let parts = TestParts::new();
    let response: Response<String> = Response::from_parts(parts, String::from("some string"));
    
    // This will panic since we're asserting a wrong value
    let _mapped_response: Response<Vec<u8>> = response.map(|b| {
        assert_eq!(b, "wrong value");
        b.as_bytes().to_vec()
    });
}

