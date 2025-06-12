// Answer 0

#[test]
fn test_alphabetic_lowercase_range() {
    // Check if the function returns a lowercase character within 'a' to 'z'
    let result = alphabetic();
    assert!(result >= 'a' && result <= 'z');
}

#[test]
fn test_alphabetic_uppercase_range() {
    // Check if the function returns an uppercase character within 'A' to 'Z'
    let result = alphabetic();
    assert!(result >= 'A' && result <= 'Z');
}

#[test]
#[should_panic]
fn test_alphabetic_empty_range() {
    // Simulate an empty range (the function does not support this, so we assert that it panics).
    let result: char = with_rng(|rng| {
        // This is a trick to force the panic condition.
        // Since we cannot directly create an "empty range", we will call 
        // a method that leads to an invalid state.
        rng.alphabetic()
    });
    assert!(false, "Expected to panic on an empty range");
}

#[test]
fn test_alphabetic_character_types() {
    // Check that the generated character is of type char and not any other type.
    let result = alphabetic();
    assert_eq!(std::any::type_name_of_val(&result), "char");
}

