// Answer 0

#[test]
fn test_generic_jaro_winkler_high_similarities() {
    let string_a = "examples";
    let string_b = "exampler";
    let result = generic_jaro_winkler(&string_a, &string_b);
    assert_eq!(result, 0.8766666666666667); // 0.7 + 0.1 * 3 * (1.0 - 0.7)
}

#[test]
fn test_generic_jaro_winkler_no_prefix() {
    let string_a = "abcdef";
    let string_b = "xyzabcde";
    let result = generic_jaro_winkler(&string_a, &string_b);
    assert_eq!(result, 0.0); // sim will be calculated to be <= 0.7
}

#[test]
fn test_generic_jaro_winkler_partial_match() {
    let string_a = "similarity";
    let string_b = "similartity";
    let result = generic_jaro_winkler(&string_a, &string_b);
    assert!(result > 0.7); // ensured based on expected values
}

#[test]
fn test_generic_jaro_winkler_max_prefix() {
    let string_a = "prefixes";
    let string_b = "prefixes123";
    let result = generic_jaro_winkler(&string_a, &string_b);
    assert_eq!(result, 1.0); // full match + counts for 4 common prefix characters
}

#[test]
fn test_generic_jaro_winkler_high_similarity_with_differences() {
    let string_a = "kitten";
    let string_b = "sitting";
    let result = generic_jaro_winkler(&string_a, &string_b);
    assert!(result > 0.7); // increased similarity due to prefix and structure
}

#[test]
fn test_generic_jaro_winkler_empty_strings() {
    let string_a = "";
    let string_b = "";
    let result = generic_jaro_winkler(&string_a, &string_b);
    assert_eq!(result, 1.0); // both are empty, should return 1.0
}

