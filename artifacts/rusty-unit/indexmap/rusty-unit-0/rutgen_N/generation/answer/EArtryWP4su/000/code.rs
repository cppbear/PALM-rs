// Answer 0

#[test]
fn test_set_difference_non_empty() {
    use indexmap::IndexSet;

    let set_a: IndexSet<i32> = [1, 2, 3, 4].iter().cloned().collect();
    let set_b: IndexSet<i32> = [3, 4, 5].iter().cloned().collect();

    let result = set_a.sub(&set_b);
    let expected: IndexSet<i32> = [1, 2].iter().cloned().collect();

    assert_eq!(result, expected);
}

#[test]
fn test_set_difference_empty_other() {
    use indexmap::IndexSet;

    let set_a: IndexSet<i32> = [1, 2, 3].iter().cloned().collect();
    let set_b: IndexSet<i32> = [].iter().cloned().collect();

    let result = set_a.sub(&set_b);
    let expected: IndexSet<i32> = [1, 2, 3].iter().cloned().collect();

    assert_eq!(result, expected);
}

#[test]
fn test_set_difference_empty_self() {
    use indexmap::IndexSet;

    let set_a: IndexSet<i32> = [].iter().cloned().collect();
    let set_b: IndexSet<i32> = [3, 4].iter().cloned().collect();

    let result = set_a.sub(&set_b);
    let expected: IndexSet<i32> = [].iter().cloned().collect();

    assert_eq!(result, expected);
}

#[test]
fn test_set_difference_no_common_elements() {
    use indexmap::IndexSet;

    let set_a: IndexSet<i32> = [1, 2].iter().cloned().collect();
    let set_b: IndexSet<i32> = [3, 4].iter().cloned().collect();

    let result = set_a.sub(&set_b);
    let expected: IndexSet<i32> = [1, 2].iter().cloned().collect();

    assert_eq!(result, expected);
}

#[test]
fn test_set_difference_all_common_elements() {
    use indexmap::IndexSet;

    let set_a: IndexSet<i32> = [1, 2, 3].iter().cloned().collect();
    let set_b: IndexSet<i32> = [1, 2, 3].iter().cloned().collect();

    let result = set_a.sub(&set_b);
    let expected: IndexSet<i32> = [].iter().cloned().collect();

    assert_eq!(result, expected);
}

