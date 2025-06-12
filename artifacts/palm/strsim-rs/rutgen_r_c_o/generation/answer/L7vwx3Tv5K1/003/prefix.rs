// Answer 0

#[test]
fn test_osa_distance_case_1() {
    let a = "abcde";
    let b = "bcdea";
    osa_distance(a, b);
}

#[test]
fn test_osa_distance_case_2() {
    let a = "hello";
    let b = "billion";
    osa_distance(a, b);
}

#[test]
fn test_osa_distance_case_3() {
    let a = "test";
    let b = "sett";
    osa_distance(a, b);
}

#[test]
fn test_osa_distance_case_4() {
    let a = "a";
    let b = "b";
    osa_distance(a, b);
}

#[test]
fn test_osa_distance_edge_case_empty_a() {
    let a = "";
    let b = "nonempty";
    osa_distance(a, b);
}

#[test]
fn test_osa_distance_edge_case_empty_b() {
    let a = "nonempty";
    let b = "";
    osa_distance(a, b);
}

#[test]
fn test_osa_distance_same_strings() {
    let a = "same";
    let b = "same";
    osa_distance(a, b);
}

#[test]
fn test_osa_distance_case_special_characters() {
    let a = "a1b2c3";
    let b = "1ab2c3";
    osa_distance(a, b);
}

#[test]
fn test_osa_distance_long_difference() {
    let a = "abcdefghij";
    let b = "zyxwvutsrq";
    osa_distance(a, b);
}

