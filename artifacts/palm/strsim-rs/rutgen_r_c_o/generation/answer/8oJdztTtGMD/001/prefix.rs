// Answer 0

#[test]
fn test_jaro_winkler_empty_strings() {
    jaro_winkler("", "");
}

#[test]
fn test_jaro_winkler_equal_strings() {
    jaro_winkler("test", "test");
}

#[test]
fn test_jaro_winkler_completely_different_strings() {
    jaro_winkler("abc", "def");
}

#[test]
fn test_jaro_winkler_partial_match() {
    jaro_winkler("hello", "helium");
}

#[test]
fn test_jaro_winkler_common_prefix() {
    jaro_winkler("common", "commonprefix");
}

#[test]
fn test_jaro_winkler_special_characters() {
    jaro_winkler("test@123", "test@456");
}

#[test]
fn test_jaro_winkler_common_prefix_edge() {
    jaro_winkler("prefix", "pref");
}

#[test]
fn test_jaro_winkler_numeric_strings() {
    jaro_winkler("12345", "12345");
}

#[test]
fn test_jaro_winkler_mixed_content() {
    jaro_winkler("Data123!", "Data321!");
}

#[test]
fn test_jaro_winkler_empty_and_non_empty_string() {
    jaro_winkler("", "nonempty");
}

