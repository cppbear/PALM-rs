// Answer 0

#[test]
fn test_peek_invalid_type_n() {
    let read = /* initialize your Read implementation */;
    let mut deserializer = Deserializer { read, scratch: Vec::new(), remaining_depth: 128 };
    deserializer.peek_or_null = || Ok(Some(b'n'));
    let expected_exp: &dyn Expected = /* initialize your Expected implementation */;
    deserializer.peek_invalid_type(expected_exp);
}

#[test]
fn test_peek_invalid_type_f() {
    let read = /* initialize your Read implementation */;
    let mut deserializer = Deserializer { read, scratch: Vec::new(), remaining_depth: 128 };
    deserializer.peek_or_null = || Ok(Some(b'f'));
    let expected_exp: &dyn Expected = /* initialize your Expected implementation */;
    deserializer.peek_invalid_type(expected_exp);
}

#[test]
fn test_peek_invalid_type_t() {
    let read = /* initialize your Read implementation */;
    let mut deserializer = Deserializer { read, scratch: Vec::new(), remaining_depth: 128 };
    deserializer.peek_or_null = || Ok(Some(b't'));
    let expected_exp: &dyn Expected = /* initialize your Expected implementation */;
    deserializer.peek_invalid_type(expected_exp);
}

#[test]
fn test_peek_invalid_type_minus() {
    let read = /* initialize your Read implementation */;
    let mut deserializer = Deserializer { read, scratch: Vec::new(), remaining_depth: 128 };
    deserializer.peek_or_null = || Ok(Some(b'-'));
    let expected_exp: &dyn Expected = /* initialize your Expected implementation */;
    deserializer.peek_invalid_type(expected_exp);
}

#[test]
fn test_peek_invalid_type_zero() {
    let read = /* initialize your Read implementation */;
    let mut deserializer = Deserializer { read, scratch: Vec::new(), remaining_depth: 128 };
    deserializer.peek_or_null = || Ok(Some(b'0'));
    let expected_exp: &dyn Expected = /* initialize your Expected implementation */;
    deserializer.peek_invalid_type(expected_exp);
}

#[test]
fn test_peek_invalid_type_one() {
    let read = /* initialize your Read implementation */;
    let mut deserializer = Deserializer { read, scratch: Vec::new(), remaining_depth: 128 };
    deserializer.peek_or_null = || Ok(Some(b'1'));
    let expected_exp: &dyn Expected = /* initialize your Expected implementation */;
    deserializer.peek_invalid_type(expected_exp);
}

#[test]
fn test_peek_invalid_type_nine() {
    let read = /* initialize your Read implementation */;
    let mut deserializer = Deserializer { read, scratch: Vec::new(), remaining_depth: 128 };
    deserializer.peek_or_null = || Ok(Some(b'9'));
    let expected_exp: &dyn Expected = /* initialize your Expected implementation */;
    deserializer.peek_invalid_type(expected_exp);
}

#[test]
fn test_peek_invalid_type_quote() {
    let read = /* initialize your Read implementation */;
    let mut deserializer = Deserializer { read, scratch: Vec::new(), remaining_depth: 128 };
    deserializer.peek_or_null = || Ok(Some(b'"'));
    let expected_exp: &dyn Expected = /* initialize your Expected implementation */;
    deserializer.peek_invalid_type(expected_exp);
}

#[test]
fn test_peek_invalid_type_square_bracket() {
    let read = /* initialize your Read implementation */;
    let mut deserializer = Deserializer { read, scratch: Vec::new(), remaining_depth: 128 };
    deserializer.peek_or_null = || Ok(Some(b'['));
    let expected_exp: &dyn Expected = /* initialize your Expected implementation */;
    deserializer.peek_invalid_type(expected_exp);
}

#[test]
fn test_peek_invalid_type_curly_bracket() {
    let read = /* initialize your Read implementation */;
    let mut deserializer = Deserializer { read, scratch: Vec::new(), remaining_depth: 128 };
    deserializer.peek_or_null = || Ok(Some(b'{'));
    let expected_exp: &dyn Expected = /* initialize your Expected implementation */;
    deserializer.peek_invalid_type(expected_exp);
}

