// Answer 0

#[test]
fn test_new_with_non_overlapping_sets() {
    use indexmap::IndexSet;

    let set1: IndexSet<i32> = IndexSet::from_iter(vec![1, 2, 3]);
    let set2: IndexSet<i32> = IndexSet::from_iter(vec![4, 5, 6]);
    
    let result = new(&set1, &set2);
    let expected: Vec<i32> = vec![1, 2, 3, 4, 5, 6];
    let result_vec: Vec<i32> = result.iter.collect();

    assert_eq!(result_vec, expected);
}

#[test]
fn test_new_with_identical_sets() {
    use indexmap::IndexSet;

    let set1: IndexSet<i32> = IndexSet::from_iter(vec![1, 2, 3]);
    let set2: IndexSet<i32> = IndexSet::from_iter(vec![1, 2, 3]);
    
    let result = new(&set1, &set2);
    let expected: Vec<i32> = vec![];
    let result_vec: Vec<i32> = result.iter.collect();

    assert_eq!(result_vec, expected);
}

#[test]
fn test_new_with_empty_and_non_empty_set() {
    use indexmap::IndexSet;

    let set1: IndexSet<i32> = IndexSet::new();
    let set2: IndexSet<i32> = IndexSet::from_iter(vec![4, 5, 6]);
    
    let result = new(&set1, &set2);
    let expected: Vec<i32> = vec![4, 5, 6];
    let result_vec: Vec<i32> = result.iter.collect();

    assert_eq!(result_vec, expected);
}

#[test]
fn test_new_with_empty_sets() {
    use indexmap::IndexSet;

    let set1: IndexSet<i32> = IndexSet::new();
    let set2: IndexSet<i32> = IndexSet::new();
    
    let result = new(&set1, &set2);
    let expected: Vec<i32> = vec![];
    let result_vec: Vec<i32> = result.iter.collect();

    assert_eq!(result_vec, expected);
}

#[test]
fn test_new_with_partial_overlap() {
    use indexmap::IndexSet;

    let set1: IndexSet<i32> = IndexSet::from_iter(vec![1, 2, 3, 7]);
    let set2: IndexSet<i32> = IndexSet::from_iter(vec![3, 4, 5]);
    
    let result = new(&set1, &set2);
    let expected: Vec<i32> = vec![1, 2, 4, 5, 7];
    let result_vec: Vec<i32> = result.iter.collect();

    assert_eq!(result_vec, expected);
}

