// Answer 0

#[test]
fn test_empty_a_elems() {
    let a_elems: Vec<i32> = vec![];
    let b_elems = vec![1, 2, 3];
    generic_damerau_levenshtein(&a_elems, &b_elems);
}

#[test]
fn test_empty_b_elems() {
    let a_elems = vec![1, 2, 3];
    let b_elems: Vec<i32> = vec![];
    generic_damerau_levenshtein(&a_elems, &b_elems);
}

#[test]
fn test_both_empty() {
    let a_elems: Vec<i32> = vec![];
    let b_elems: Vec<i32> = vec![];
    generic_damerau_levenshtein(&a_elems, &b_elems);
}

#[test]
fn test_simple_damerau() {
    let a_elems = vec![1, 2, 3];
    let b_elems = vec![3, 2, 1];
    generic_damerau_levenshtein(&a_elems, &b_elems);
}

#[test]
fn test_identical_elems() {
    let a_elems = vec![1, 2, 3];
    let b_elems = vec![1, 2, 3];
    generic_damerau_levenshtein(&a_elems, &b_elems);
}

#[test]
fn test_one_insert() {
    let a_elems = vec![1, 2];
    let b_elems = vec![1, 2, 3];
    generic_damerau_levenshtein(&a_elems, &b_elems);
}

#[test]
fn test_one_delete() {
    let a_elems = vec![1, 2, 3];
    let b_elems = vec![1, 2];
    generic_damerau_levenshtein(&a_elems, &b_elems);
}

#[test]
fn test_two_substitutions() {
    let a_elems = vec![1, 2, 3];
    let b_elems = vec![3, 4, 5];
    generic_damerau_levenshtein(&a_elems, &b_elems);
}

#[test]
fn test_transposition() {
    let a_elems = vec![1, 2];
    let b_elems = vec![2, 1];
    generic_damerau_levenshtein(&a_elems, &b_elems);
}

