// Answer 0

#[test]
fn test_osa_distance_case1() {
    let a = "a";
    let b = "b";
    let result = osa_distance(a, b);
}

#[test]
fn test_osa_distance_case2() {
    let a = "ab";
    let b = "bc";
    let result = osa_distance(a, b);
}

#[test]
fn test_osa_distance_case3() {
    let a = "abc";
    let b = "bca";
    let result = osa_distance(a, b);
}

#[test]
fn test_osa_distance_case4() {
    let a = "abcd";
    let b = "bcda";
    let result = osa_distance(a, b);
}

#[test]
fn test_osa_distance_case5() {
    let a = "abcde";
    let b = "edcba";
    let result = osa_distance(a, b);
}

#[test]
fn test_osa_distance_case6() {
    let a = "abcdefghijk";
    let b = "kljihgfedcba";
    let result = osa_distance(a, b);
}

#[test]
fn test_osa_distance_case7() {
    let a = "rust";
    let b = "tsur";
    let result = osa_distance(a, b);
}

#[test]
fn test_osa_distance_empty_strings() {
    let a = "";
    let b = "";
    let result = osa_distance(a, b);
}

#[test]
fn test_osa_distance_one_empty_string() {
    let a = "a";
    let b = "";
    let result = osa_distance(a, b);
}

#[test]
fn test_osa_distance_long_strings() {
    let a = "a".repeat(1000);
    let b = "b".repeat(1000);
    let result = osa_distance(a, b);
}

