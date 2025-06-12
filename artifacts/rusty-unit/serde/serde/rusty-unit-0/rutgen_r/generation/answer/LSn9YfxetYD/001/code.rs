// Answer 0

#[derive(Debug)]
struct MyStruct(String);

impl MyStruct {
    fn visit_bytes<E>(&mut self, v: &[u8]) -> Result<(), E>
    where
        E: std::fmt::Debug,
    {
        match std::str::from_utf8(v) {
            Ok(s) => {
                self.0.clear();
                self.0.push_str(s);
                Ok(())
            }
            Err(_) => Err(E::invalid_value(Unexpected::Bytes(v), &self)),
        }
    }
}

#[test]
fn test_visit_bytes_err() {
    struct TestError;

    impl std::fmt::Debug for TestError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "TestError")
        }
    }

    impl From<&[u8]> for TestError {
        fn from(_: &[u8]) -> Self {
            TestError
        }
    }

    let mut my_struct = MyStruct(String::new());
    let input: &[u8] = b"\xFF"; // Invalid UTF-8 byte
    let result: Result<(), TestError> = my_struct.visit_bytes(input);
    assert!(result.is_err());
}

