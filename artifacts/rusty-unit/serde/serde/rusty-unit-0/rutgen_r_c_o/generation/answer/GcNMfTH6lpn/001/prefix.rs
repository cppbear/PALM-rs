// Answer 0

#[test]
fn test_visit_string_empty() {
    let mut target = String::new();
    let input = String::new();
    StringInPlaceVisitor(&mut target).visit_string(input).unwrap();
}

#[test]
fn test_visit_string_small() {
    let mut target = String::new();
    let input = String::from("Hello");
    StringInPlaceVisitor(&mut target).visit_string(input).unwrap();
}

#[test]
fn test_visit_string_medium() {
    let mut target = String::new();
    let input = String::from("This is a string for testing.");
    StringInPlaceVisitor(&mut target).visit_string(input).unwrap();
}

#[test]
fn test_visit_string_large() {
    let mut target = String::new();
    let input = String::from("A".repeat(1000));
    StringInPlaceVisitor(&mut target).visit_string(input).unwrap();
}

#[test]
#[should_panic]
fn test_visit_string_invalid_utf8() {
    let mut target = String::new();
    let input = String::from_utf8_lossy(&[0, 159, 146, 150]).into_owned(); // Invalid UTF-8 sequence
    StringInPlaceVisitor(&mut target).visit_string(input).unwrap();
}

