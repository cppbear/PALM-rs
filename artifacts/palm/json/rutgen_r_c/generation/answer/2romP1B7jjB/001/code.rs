// Answer 0

#[test]
fn test_from_str_positive_integer() {
    let result = Number::from_str("12345");
    assert!(result.is_ok());
    assert_eq!(result.unwrap().n, N::PosInt(12345));
}

#[test]
fn test_from_str_negative_integer() {
    let result = Number::from_str("-12345");
    assert!(result.is_ok());
    assert_eq!(result.unwrap().n, N::NegInt(-12345));
}

#[test]
fn test_from_str_positive_float() {
    let result = Number::from_str("123.45");
    assert!(result.is_ok());
    if let Number { n: N::Float(value) } = result.unwrap() {
        assert_eq!(value, 123.45);
    } else {
        panic!("Expected float but got a different type");
    }
}

#[test]
fn test_from_str_negative_float() {
    let result = Number::from_str("-123.45");
    assert!(result.is_ok());
    if let Number { n: N::Float(value) } = result.unwrap() {
        assert_eq!(value, -123.45);
    } else {
        panic!("Expected float but got a different type");
    }
}

#[test]
#[should_panic]
fn test_from_str_invalid_input_not_a_number() {
    let _ = Number::from_str("abc");
}

#[test]
#[should_panic]
fn test_from_str_invalid_input_empty_string() {
    let _ = Number::from_str("");
}

#[test]
#[should_panic]
fn test_from_str_invalid_input_special_characters() {
    let _ = Number::from_str("$^&*");
}

#[test]
fn test_from_str_zero() {
    let result = Number::from_str("0");
    assert!(result.is_ok());
    assert_eq!(result.unwrap().n, N::PosInt(0));
}

