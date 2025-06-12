// Answer 0

#[derive(Default)]
struct MockVisitor {
    value: Option<u128>,
}

impl<'any> de::Visitor<'any> for MockVisitor {
    type Value = Result<u128, &'static str>;

    fn visit_u128(self, value: u128) -> Self::Value {
        Ok(value)
    }
}

struct MockDeserializer {
    whitespace_to_return: Option<u8>,
    scan_result: Result<(), &'static str>,
}

impl MockDeserializer {
    fn parse_whitespace(&mut self) -> Result<Option<u8>, &'static str> {
        Ok(self.whitespace_to_return)
    }

    fn scan_integer128(&mut self, buf: &mut String) -> Result<(), &'static str> {
        if self.scan_result.is_ok() {
            buf.push_str("123456789012345678901234567890"); // Valid u128 number
            Ok(())
        } else {
            self.scan_result
        }
    }
}

#[test]
fn test_success_case() {
    let mut deserializer = MockDeserializer {
        whitespace_to_return: Some(b' '),
        scan_result: Ok(()),
    };
    
    let mut visitor = MockVisitor::default();
    let result = deserializer.do_deserialize_u128(&mut visitor);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 123456789012345678901234567890);
}

#[test]
fn test_negative_number() {
    let mut deserializer = MockDeserializer {
        whitespace_to_return: Some(b'-'),
        scan_result: Ok(()),
    };
    
    let mut visitor = MockVisitor::default();
    let result = deserializer.do_deserialize_u128(&mut visitor);
    assert!(result.is_err());
}

#[test]
fn test_eof_while_parsing_value() {
    let mut deserializer = MockDeserializer {
        whitespace_to_return: None,
        scan_result: Ok(()),
    };

    let mut visitor = MockVisitor::default();
    let result = deserializer.do_deserialize_u128(&mut visitor);
    assert!(result.is_err());
}

#[test]
fn test_scan_integer_error() {
    let mut deserializer = MockDeserializer {
        whitespace_to_return: Some(b' '),
        scan_result: Err("scan error"),
    };

    let mut visitor = MockVisitor::default();
    let result = deserializer.do_deserialize_u128(&mut visitor);
    assert!(result.is_err());
}

#[test]
fn test_buf_parse_failure() {
    let mut deserializer = MockDeserializer {
        whitespace_to_return: Some(b' '),
        scan_result: Ok(()),
    };

    let mut visitor = MockVisitor {
        value: None,
    };
    visitor.value = Some(0); // intentionally set to not match
    let result = deserializer.do_deserialize_u128(&mut visitor);
    assert!(result.is_err());
}

