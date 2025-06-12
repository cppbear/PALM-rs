// Answer 0

#[test]
fn test_unexpected_float() {
    let content = Content::F64(3.14);
    let result = content.unexpected();
    match result {
        Unexpected::Float(f) => assert_eq!(f, 3.14),
        _ => panic!("Expected Unexpected::Float(3.14)"),
    }
}

#[test]
fn test_unexpected_float_zero() {
    let content = Content::F64(0.0);
    let result = content.unexpected();
    match result {
        Unexpected::Float(f) => assert_eq!(f, 0.0),
        _ => panic!("Expected Unexpected::Float(0.0)"),
    }
}

#[test]
fn test_unexpected_negative_float() {
    let content = Content::F64(-2.71);
    let result = content.unexpected();
    match result {
        Unexpected::Float(f) => assert_eq!(f, -2.71),
        _ => panic!("Expected Unexpected::Float(-2.71)"),
    }
}

