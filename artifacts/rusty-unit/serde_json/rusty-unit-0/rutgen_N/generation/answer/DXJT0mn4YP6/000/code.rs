// Answer 0

#[test]
fn test_parse_str_raw_success() {
    struct MockDelegate;

    impl MockDelegate {
        fn parse_str_raw<'s>(
            &'s mut self,
            scratch: &'s mut Vec<u8>,
        ) -> Result<&'s [u8], &'static str> {
            scratch.extend_from_slice(b"test");
            Ok(&scratch[..])
        }
    }

    struct Parser<'a> {
        delegate: MockDelegate,
    }

    let mut scratch = Vec::new();
    let mut parser = Parser {
        delegate: MockDelegate,
    };

    let result = parser.parse_str_raw(&mut scratch);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), b"test");
}

#[test]
#[should_panic(expected = "some error message")]
fn test_parse_str_raw_failure() {
    struct MockDelegate;

    impl MockDelegate {
        fn parse_str_raw<'s>(
            &'s mut self,
            scratch: &'s mut Vec<u8>,
        ) -> Result<&'s [u8], &'static str> {
            Err("some error message")
        }
    }

    struct Parser<'a> {
        delegate: MockDelegate,
    }

    let mut scratch = Vec::new();
    let mut parser = Parser {
        delegate: MockDelegate,
    };

    let _ = parser.parse_str_raw(&mut scratch).unwrap();
}

