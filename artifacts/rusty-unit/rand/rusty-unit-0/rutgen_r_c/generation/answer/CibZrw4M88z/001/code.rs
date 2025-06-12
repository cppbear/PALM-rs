// Answer 0

#[test]
fn test_borrow_for_reference() {
    struct TestStruct;
    
    let test_value = TestStruct;
    let reference: &TestStruct = &test_value;
    
    assert_eq!(reference.borrow(), &test_value);
}

#[test]
fn test_borrow_for_integer_reference() {
    let value = 42;
    let reference: &i32 = &value;

    assert_eq!(reference.borrow(), &value);
}

#[test]
fn test_borrow_for_string_reference() {
    let value = String::from("test");
    let reference: &String = &value;

    assert_eq!(reference.borrow(), &value);
}

#[test]
fn test_borrow_for_char_reference() {
    let value = 'a';
    let reference: &char = &value;

    assert_eq!(reference.borrow(), &value);
}

#[test]
fn test_borrow_for_float_reference() {
    let value = 3.14;
    let reference: &f64 = &value;

    assert_eq!(reference.borrow(), &value);
}

