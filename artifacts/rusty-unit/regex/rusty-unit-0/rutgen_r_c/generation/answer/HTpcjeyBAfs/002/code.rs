// Answer 0

#[test]
fn test_ages_invalid_property_value() {
    // Test with a canonical_age that does not match any predefined ages
    let result = ages("Invalid_Age");
    assert_eq!(result, Err(Error::PropertyValueNotFound));
}

#[test]
fn test_ages_empty_string() {
    // Test with an empty string as the canonical_age
    let result = ages("");
    assert_eq!(result, Err(Error::PropertyValueNotFound));
}

#[test]
fn test_ages_numeric_string() {
    // Test with a numeric string that does not match any predefined ages
    let result = ages("1234");
    assert_eq!(result, Err(Error::PropertyValueNotFound));
}

#[test]
fn test_ages_special_characters() {
    // Test with a string containing special characters
    let result = ages("@#$%^&*()");
    assert_eq!(result, Err(Error::PropertyValueNotFound));
}

