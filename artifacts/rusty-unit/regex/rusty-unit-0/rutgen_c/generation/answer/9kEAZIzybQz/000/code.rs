// Answer 0

#[test]
fn test_canonical_script_valid_input() {
    struct PropertyValuesStub;
    
    impl PropertyValuesStub {
        fn new() -> PropertyValues {
            // Here we would initialize and return a mock PropertyValues for the script.
            // Assuming `PropertyValues` is just a placeholder struct.
            // In a real implementation, you would provide a valid instance according
            // to the context.
            PropertyValues { /* initialization details */ }
        }
    }

    let normalized_value = "Latin"; // example of a valid normalized script value
    let result = canonical_script(normalized_value);
    assert_eq!(result, Some("Latn")); // Assuming "Latn" is the expected return for "Latin"
}

#[test]
fn test_canonical_script_invalid_input() {
    let normalized_value = "InvalidScript"; // example of an invalid normalized script value
    let result = canonical_script(normalized_value);
    assert_eq!(result, None); // Expect None for invalid script values
}

#[test]
fn test_canonical_script_empty_input() {
    let normalized_value = ""; // test with empty string
    let result = canonical_script(normalized_value);
    assert_eq!(result, None); // Expect None for empty input
}

