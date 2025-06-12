// Answer 0

#[test]
fn test_fmt_with_unit() {
    use serde::de;

    struct TestUnexpected(de::Unexpected<'static>);
    
    let unexpected = TestUnexpected(de::Unexpected::Unit);
    let mut output = String::new();
    let result = unexpected.0.fmt(&mut output).map(|_| output.clone());

    assert_eq!(result, Ok(String::from("null")));
}

#[test]
fn test_fmt_with_float() {
    use serde::de;

    struct TestUnexpected(de::Unexpected<'static>);
    
    let value: f64 = 3.14;
    let unexpected = TestUnexpected(de::Unexpected::Float(value));
    let mut output = String::new();
    let result = unexpected.0.fmt(&mut output).map(|_| output.clone());

    assert_eq!(result.is_ok(), true);
    assert!(output.contains("floating point `3.14`"));
}

#[test]
fn test_fmt_with_other() {
    use serde::de;

    struct TestUnexpected(de::Unexpected<'static>);
    
    let unexpected = TestUnexpected(de::Unexpected::Bytes(b"some_bytes"));
    let mut output = String::new();
    let result = unexpected.0.fmt(&mut output).map(|_| output.clone());

    assert_eq!(result.is_ok(), true);
    assert!(output.contains("some_bytes"));
}

