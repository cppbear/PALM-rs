// Answer 0

#[test]
fn test_lowercase() {
    let result = fastrand::lowercase();
    assert!(result.is_ascii_lowercase(), "The generated character is not a lowercase ASCII letter.");
    assert!((result >= 'a' && result <= 'z'), "The generated character is out of the expected range a-z.");
}

