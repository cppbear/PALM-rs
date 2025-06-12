// Answer 0

#[test]
fn test_osa_distance_empty_strings() {
    let a = "";
    let b = "";
    osa_distance(a, b);
}

#[test]
fn test_osa_distance_one_empty_string() {
    let a = "abc";
    let b = "";
    osa_distance(a, b);
}

#[test]
fn test_osa_distance_both_single_char_same() {
    let a = "a";
    let b = "a";
    osa_distance(a, b);
}

#[test]
fn test_osa_distance_both_single_char_different() {
    let a = "a";
    let b = "b";
    osa_distance(a, b);
}

#[test]
fn test_osa_distance_multiple_same_chars() {
    let a = "aaaa";
    let b = "aaaa";
    osa_distance(a, b);
}

#[test]
fn test_osa_distance_all_different_chars() {
    let a = "abcd";
    let b = "efgh";
    osa_distance(a, b);
}

#[test]
fn test_osa_distance_some_different() {
    let a = "abcde";
    let b = "abfgh";
    osa_distance(a, b);
}

#[test]
fn test_osa_distance_with_transposition() {
    let a = "abc";
    let b = "acb";
    osa_distance(a, b);
}

#[test]
fn test_osa_distance_edge_case_one_char_different() {
    let a = "a";
    let b = "ab";
    osa_distance(a, b);
}

#[test]
fn test_osa_distance_edge_case_same_length_different_one() {
    let a = "abc";
    let b = "abx";
    osa_distance(a, b);
}

#[test]
fn test_osa_distance_longer_strings() {
    let a = "abcdefghij";
    let b = "abcdefghij";
    osa_distance(a, b);
}

#[test]
fn test_osa_distance_long_strings_different() {
    let a = "abcdefghij";
    let b = "klmnopqrst";
    osa_distance(a, b);
}

