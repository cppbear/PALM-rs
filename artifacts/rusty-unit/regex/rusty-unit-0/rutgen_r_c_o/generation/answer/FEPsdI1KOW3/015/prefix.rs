// Answer 0

#[test]
fn test_from_name_invalid_empty() {
    from_name("");
}

#[test]
fn test_from_name_invalid_not_a_name() {
    from_name("not_a_name");
}

#[test]
fn test_from_name_invalid_random_word() {
    from_name("random");
}

#[test]
fn test_from_name_invalid_special_characters() {
    from_name("@#%^&*");
}

#[test]
fn test_from_name_invalid_numeric_string() {
    from_name("12345");
}

#[test]
fn test_from_name_invalid_long_string() {
    from_name("this_is_a_very_long_string_that_should_return_none");
}

#[test]
fn test_from_name_invalid_single_character() {
    from_name("x");
}

#[test]
fn test_from_name_invalid_multiple_characters() {
    from_name("xyz");
}

#[test]
fn test_from_name_invalid_capitalization() {
    from_name("Alphabet");
}

