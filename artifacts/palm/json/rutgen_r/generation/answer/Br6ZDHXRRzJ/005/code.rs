// Answer 0

fn test_unexpected_bool_true() {
    let value = Value::Bool(true);
    let result = value.unexpected();
    match result {
        Unexpected::Bool(b) => assert_eq!(b, true),
        _ => panic!("Expected Unexpected::Bool(true), but got {:?}", result),
    }
}

fn test_unexpected_bool_false() {
    let value = Value::Bool(false);
    let result = value.unexpected();
    match result {
        Unexpected::Bool(b) => assert_eq!(b, false),
        _ => panic!("Expected Unexpected::Bool(false), but got {:?}", result),
    }
}

