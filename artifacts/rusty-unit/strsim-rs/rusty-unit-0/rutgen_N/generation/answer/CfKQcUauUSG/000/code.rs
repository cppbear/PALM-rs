// Answer 0

#[cfg(test)]
use std::collections::HashMap;
use std::cmp::min;
use std::hash::Hash;

fn flat_index(i: usize, j: usize, width: usize) -> usize {
    i * width + j
}

#[test]
fn test_generic_damerau_levenshtein_case1() {
    let a = &[1, 2];
    let b = &[2, 3, 1];
    assert_eq!(2, generic_damerau_levenshtein(a, b));
}

#[test]
fn test_generic_damerau_levenshtein_case2() {
    let a: &[char] = &['a', 'b'];
    let b: &[char] = &['b', 'c', 'a'];
    assert_eq!(2, generic_damerau_levenshtein(a, b));
}

#[test]
fn test_generic_damerau_levenshtein_empty_a() {
    let a: &[i32] = &[];
    let b: &[i32] = &[1, 2, 3];
    assert_eq!(3, generic_damerau_levenshtein(a, b));
}

#[test]
fn test_generic_damerau_levenshtein_empty_b() {
    let a: &[i32] = &[1, 2, 3];
    let b: &[i32] = &[];
    assert_eq!(3, generic_damerau_levenshtein(a, b));
}

#[test]
fn test_generic_damerau_levenshtein_case3() {
    let a = &[1, 1, 1];
    let b = &[1, 1, 2];
    assert_eq!(1, generic_damerau_levenshtein(a, b));
}

#[test]
fn test_generic_damerau_levenshtein_case4() {
    let a = &["test", "string"];
    let b = &["text", "string"];
    assert_eq!(1, generic_damerau_levenshtein(a, b));
}

