// Answer 0

#[test]
fn test_peek_invalid_type_n_failure() {
    struct MockExpected;

    impl Expected for MockExpected {}

    struct DeMock {
        data: Vec<u8>,
        position: usize,
        scratch: Vec<u8>,
    }

    impl DeMock {
        fn peek_or_null(&self) -> Option<u8> {
            self.data.get(self.position).cloned()
        }

        fn eat_char(&mut self) {
            self.position += 1;
        }

        fn parse_ident(&self, _: &[u8]) -> Result<(), Error> {
            Err(Error)
        }

        fn fix_position(&self, err: Error) -> Error {
            err
        }

        fn peek_error(&self, _: ErrorCode) -> Error {
            Error
        }

        fn parse_any_number(&self, _: bool) -> Result<NumberMock, Error> {
            Err(Error)
        }
    }

    impl DeMock {
        fn new(data: Vec<u8>) -> Self {
            DeMock {
                data,
                position: 0,
                scratch: vec![],
            }
        }
    }

    struct NumberMock;

    impl NumberMock {
        fn invalid_type(self, _: &dyn Expected) -> Error {
            Error
        }
    }

    let mut deser = DeMock::new(vec![b'n']);
    let expected = MockExpected;
    
    let result = deser.peek_invalid_type(&expected);

    // Asserting that the result is the error returned from parse_ident
    assert!(result.is_err());
}

