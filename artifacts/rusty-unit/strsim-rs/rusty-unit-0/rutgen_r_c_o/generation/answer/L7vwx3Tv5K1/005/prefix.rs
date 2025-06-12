// Answer 0

#[test]
fn test_osa_distance_empty_strings() {
    let a = "";
    let b = "";
    osa_distance(a, b);
}

#[test]
fn test_osa_distance_a_empty_b_non_empty() {
    let a = "";
    let b = "abc";
    osa_distance(a, b);
}

#[test]
fn test_osa_distance_a_non_empty_b_empty() {
    let a = "abc";
    let b = "";
    osa_distance(a, b);
}

#[test]
fn test_osa_distance_identical_strings() {
    let a = "hello";
    let b = "hello";
    osa_distance(a, b);
}

#[test]
fn test_osa_distance_one_char_difference() {
    let a = "a";
    let b = "b";
    osa_distance(a, b);
}

#[test]
fn test_osa_distance_two_char_difference() {
    let a = "ab";
    let b = "cd";
    osa_distance(a, b);
}

#[test]
fn test_osa_distance_with_transposition() {
    let a = "abcd";
    let b = "abdc";
    osa_distance(a, b);
}

#[test]
fn test_osa_distance_longer_string_with_transposition() {
    let a = "abcdef";
    let b = "abdecf";
    osa_distance(a, b);
}

#[test]
fn test_osa_distance_large_strings() {
    let a = "abcdefghij";
    let b = "jihgfedcba";
    osa_distance(a, b);
}

#[test]
fn test_osa_distance_partial_overlap() {
    let a = "abcde";
    let b = "cdefg";
    osa_distance(a, b);
}

#[test]
fn test_osa_distance_full_overlap() {
    let a = "abcde";
    let b = "bcdea";
    osa_distance(a, b);
}

#[test]
fn test_osa_distance_one_insertions() {
    let a = "abc";
    let b = "abdc";
    osa_distance(a, b);
}

#[test]
fn test_osa_distance_multiple_operations() {
    let a = "kitten";
    let b = "sitting";
    osa_distance(a, b);
}

