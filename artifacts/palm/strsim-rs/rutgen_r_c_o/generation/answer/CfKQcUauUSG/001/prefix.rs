// Answer 0

#[test]
fn test_generic_damerau_levenshtein_non_empty_a_empty_b() {
    let a_elems = vec![1, 2, 3];
    let b_elems: Vec<i32> = vec![];
    generic_damerau_levenshtein(&a_elems, &b_elems);
}

#[test]
fn test_generic_damerau_levenshtein_non_empty_a_small_b() {
    let a_elems = vec![1, 2, 3];
    let b_elems = vec![2];
    generic_damerau_levenshtein(&a_elems, &b_elems);
}

#[test]
fn test_generic_damerau_levenshtein_non_empty_a_middle_b() {
    let a_elems = vec![1, 2, 3];
    let b_elems = vec![3, 1];
    generic_damerau_levenshtein(&a_elems, &b_elems);
}

#[test]
fn test_generic_damerau_levenshtein_non_empty_a_large_b() {
    let a_elems = vec![1, 2, 3];
    let b_elems = vec![3, 1, 2, 4, 5, 6, 7, 8, 9, 10];
    generic_damerau_levenshtein(&a_elems, &b_elems);
}

#[test]
fn test_generic_damerau_levenshtein_large_a_small_b() {
    let a_elems = (1..=1000).collect::<Vec<i32>>();
    let b_elems = vec![1, 2];
    generic_damerau_levenshtein(&a_elems, &b_elems);
}

#[test]
fn test_generic_damerau_levenshtein_large_a_large_b() {
    let a_elems = (1..=1000).collect::<Vec<i32>>();
    let b_elems = (1..=1000).collect::<Vec<i32>>();
    generic_damerau_levenshtein(&a_elems, &b_elems);
}

