// Answer 0

#[test]
fn test_canonical_prop_valid() {
    assert_eq!(canonical_prop("Letter"), Some("L"));
    assert_eq!(canonical_prop("Lowercase_Letter"), Some("Ll"));
    assert_eq!(canonical_prop("Uppercase_Letter"), Some("Lu"));
    assert_eq!(canonical_prop("Number"), Some("N"));
}

#[test]
fn test_canonical_prop_invalid() {
    assert_eq!(canonical_prop("Non_Existent_Property"), None);
    assert_eq!(canonical_prop("Invalid_Property_Name"), None);
}

#[test]
#[should_panic]
fn test_canonical_prop_empty_string() {
    canonical_prop("");
}

#[test]
#[should_panic]
fn test_canonical_prop_whitespace() {
    canonical_prop(" ");
}

#[test]
fn test_canonical_prop_case_sensitive() {
    assert_eq!(canonical_prop("LETTER"), None);
    assert_eq!(canonical_prop("lowercase_letter"), None);
    assert_eq!(canonical_prop("UPPERCASE_LETTER"), None);
}

