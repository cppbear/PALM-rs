// Answer 0

#[test]
fn test_generic_damerau_levenshtein_non_empty_a_empty_b() {
    let a_elems: &[i32] = &[1, 2, 3];
    let b_elems: &[i32] = &[];
    assert_eq!(3, generic_damerau_levenshtein(a_elems, b_elems));
}

#[test]
fn test_generic_damerau_levenshtein_non_empty_a_non_empty_b() {
    let a_elems: &[i32] = &[1, 2, 3];
    let b_elems: &[i32] = &[4, 5];
    assert_eq!(5, generic_damerau_levenshtein(a_elems, b_elems));
}

#[test]
fn test_generic_damerau_levenshtein_identical_strings() {
    let a_elems: &[i32] = &[1, 2, 3, 4];
    let b_elems: &[i32] = &[1, 2, 3, 4];
    assert_eq!(0, generic_damerau_levenshtein(a_elems, b_elems));
}

#[test]
fn test_generic_damerau_levenshtein_with_repetitions() {
    let a_elems: &[i32] = &[1, 1, 2, 3];
    let b_elems: &[i32] = &[1, 2, 2, 3];
    assert_eq!(2, generic_damerau_levenshtein(a_elems, b_elems));
}

#[test]
fn test_generic_damerau_levenshtein_all_different() {
    let a_elems: &[i32] = &[1, 2, 3];
    let b_elems: &[i32] = &[4, 5, 6];
    assert_eq!(6, generic_damerau_levenshtein(a_elems, b_elems));
}

