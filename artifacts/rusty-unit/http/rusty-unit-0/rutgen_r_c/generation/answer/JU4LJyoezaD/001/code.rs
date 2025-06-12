// Answer 0

#[test]
fn test_star() {
    use std::convert::TryFrom;
    use crate::byte_str::ByteStr;
    use bytes::Bytes;

    struct TestPathAndQuery;

    impl TestPathAndQuery {
        pub(super) fn star() -> super::PathAndQuery {
            super::PathAndQuery {
                data: ByteStr::from_static("*"),
                query: super::NONE,
            }
        }
    }

    let result = TestPathAndQuery::star();
    let expected = super::PathAndQuery {
        data: ByteStr::from_static("*"),
        query: super::NONE,
    };

    assert_eq!(result, expected);
}

