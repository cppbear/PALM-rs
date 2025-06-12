// Answer 0

#[derive(Debug)]
struct TestVisitor<'a>(&'a mut String);

impl<'a> serde::de::Visitor<'de> for TestVisitor<'a> {
    type Value = ();

    fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match String::from_utf8(v) {
            Ok(s) => {
                *self.0 = s;
                Ok(())
            }
            Err(e) => Err(E::invalid_value(
                serde::de::Unexpected::Bytes(&e.into_bytes()),
                &self,
            )),
        }
    }
}

#[test]
fn test_visit_byte_buf_valid() {
    let mut result_string = String::new();
    let visitor = TestVisitor(&mut result_string);
    let valid_bytes = b"hello".to_vec();

    let result = visitor.visit_byte_buf(valid_bytes);
    
    assert!(result.is_ok());
    assert_eq!(result_string, "hello");
}

#[test]
fn test_visit_byte_buf_invalid() {
    let mut result_string = String::new();
    let visitor = TestVisitor(&mut result_string);
    let invalid_bytes = vec![0, 159, 146, 150]; // invalid UTF-8 sequence

    let result: Result<(), serde::de::Error> = visitor.visit_byte_buf(invalid_bytes);

    assert!(result.is_err());
}

