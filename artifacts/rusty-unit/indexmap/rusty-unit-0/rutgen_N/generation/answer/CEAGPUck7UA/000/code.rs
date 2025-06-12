// Answer 0

#[test]
fn test_third_with_integers() {
    let tuple = (1, 2, 3);
    assert_eq!(third(tuple), 3);
}

#[test]
fn test_third_with_strings() {
    let tuple = ("first", "second", "third");
    assert_eq!(third(tuple), "third");
}

#[test]
fn test_third_with_floats() {
    let tuple = (1.1, 2.2, 3.3);
    assert_eq!(third(tuple), 3.3);
}

#[test]
fn test_third_with_mixed_types() {
    let tuple = (1, "two", 3.0);
    assert_eq!(third(tuple), 3.0);
}

#[test]
fn test_third_with_empty_tuple() {
    let tuple: (i32, i32, i32) = (0, 0, 0);
    assert_eq!(third(tuple), 0);
}

