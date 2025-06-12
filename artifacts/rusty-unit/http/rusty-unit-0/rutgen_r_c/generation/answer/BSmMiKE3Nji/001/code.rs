// Answer 0

#[test]
fn test_as_any_with_integer() {
    struct IntWrapper(i32);
    let val = IntWrapper(42);
    let any_ref: &dyn Any = val.as_any();
    assert!(any_ref.is::<IntWrapper>());
}

#[test]
fn test_as_any_with_string() {
    struct StringWrapper(String);
    let val = StringWrapper("Hello".to_string());
    let any_ref: &dyn Any = val.as_any();
    assert!(any_ref.is::<StringWrapper>());
}

#[test]
fn test_as_any_with_float() {
    struct FloatWrapper(f64);
    let val = FloatWrapper(3.14);
    let any_ref: &dyn Any = val.as_any();
    assert!(any_ref.is::<FloatWrapper>());
}

#[test]
fn test_as_any_with_bool() {
    struct BoolWrapper(bool);
    let val = BoolWrapper(true);
    let any_ref: &dyn Any = val.as_any();
    assert!(any_ref.is::<BoolWrapper>());
}

#[test]
fn test_as_any_with_vector() {
    struct VecWrapper(Vec<i32>);
    let val = VecWrapper(vec![1, 2, 3]);
    let any_ref: &dyn Any = val.as_any();
    assert!(any_ref.is::<VecWrapper>());
}

#[test]
#[should_panic]
fn test_as_any_should_panic_on_wrong_type() {
    struct WrongType;
    let val = WrongType;
    let any_ref: &dyn Any = val.as_any();
    assert!(!any_ref.is::<i32>());
}

