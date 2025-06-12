// Answer 0

#[test]
fn test_from_shared_valid_uri() {
    use bytes::Bytes;

    struct TestStruct;

    impl TryFrom<&[u8]> for PathAndQuery {
        type Error = InvalidUri;

        fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
            PathAndQuery::from_shared(Bytes::from(value))
        }
    }

    let valid_uri = b"/path/to/resource?query=value#fragment";

    let result = PathAndQuery::from_shared(Bytes::from(valid_uri.to_vec()));
    assert!(result.is_ok());

    let path_and_query = result.unwrap();
    assert_eq!(path_and_query.data.bytes, Bytes::from(valid_uri.to_vec()));
    assert_eq!(path_and_query.query, 21);
}

#[test]
fn test_from_shared_uri_with_special_characters() {
    use bytes::Bytes;

    struct TestStruct;

    impl TryFrom<&[u8]> for PathAndQuery {
        type Error = InvalidUri;

        fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
            PathAndQuery::from_shared(Bytes::from(value))
        }
    }

    let special_uri = b"/path/{test}/resource?query=value#fragment";

    let result = PathAndQuery::from_shared(Bytes::from(special_uri.to_vec()));
    assert!(result.is_ok());

    let path_and_query = result.unwrap();
    assert_eq!(path_and_query.data.bytes, Bytes::from(special_uri.to_vec()));
    assert_eq!(path_and_query.query, 14);
}

#[test]
fn test_from_shared_invalid_uri_character() {
    use bytes::Bytes;

    struct TestStruct;

    impl TryFrom<&[u8]> for PathAndQuery {
        type Error = InvalidUri;

        fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
            PathAndQuery::from_shared(Bytes::from(value))
        }
    }

    let invalid_uri = b"/path/to/resource\xFF"; // Invalid byte

    let result = PathAndQuery::from_shared(Bytes::from(invalid_uri.to_vec()));
    assert!(result.is_err());
}

#[test]
fn test_from_shared_valid_uri_with_maybe_not_utf8() {
    use bytes::Bytes;

    struct TestStruct;

    impl TryFrom<&[u8]> for PathAndQuery {
        type Error = InvalidUri;

        fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
            PathAndQuery::from_shared(Bytes::from(value))
        }
    }

    let valid_uri = b"/path/to/resource?query=value#fragment";

    let result = PathAndQuery::from_shared(Bytes::from(valid_uri.to_vec()));
    assert!(result.is_ok());
}

#[test]
fn test_from_shared_empty_input() {
    use bytes::Bytes;

    struct TestStruct;

    impl TryFrom<&[u8]> for PathAndQuery {
        type Error = InvalidUri;

        fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
            PathAndQuery::from_shared(Bytes::from(value))
        }
    }

    let empty_input = b""; 

    let result = PathAndQuery::from_shared(Bytes::from(empty_input.to_vec()));
    assert!(result.is_err());
}

