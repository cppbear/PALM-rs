// Answer 0

#[test]
fn test_intersection_basic() {
    let a: HashSet<_> = vec![1, 2, 3].into_iter().collect();
    let b: HashSet<_> = vec![2, 3, 4].into_iter().collect();
    let _set = &a & &b;
}

#[test]
fn test_intersection_empty_sets() {
    let a: HashSet<_> = vec![].into_iter().collect();
    let b: HashSet<_> = vec![].into_iter().collect();
    let _set = &a & &b;
}

#[test]
fn test_intersection_no_common_elements() {
    let a: HashSet<_> = vec![1, 2, 3].into_iter().collect();
    let b: HashSet<_> = vec![4, 5, 6].into_iter().collect();
    let _set = &a & &b;
}

#[test]
fn test_intersection_all_common_elements() {
    let a: HashSet<_> = vec![1, 2, 3, 4, 5].into_iter().collect();
    let b: HashSet<_> = vec![1, 2, 3].into_iter().collect();
    let _set = &a & &b;
}

#[test]
fn test_intersection_one_set_empty() {
    let a: HashSet<_> = vec![1, 2, 3].into_iter().collect();
    let b: HashSet<_> = vec![].into_iter().collect();
    let _set = &a & &b;
}

#[test]
fn test_intersection_large_sets() {
    let a: HashSet<_> = (0..=1000).collect();
    let b: HashSet<_> = (500..=1500).collect();
    let _set = &a & &b;
}

#[test]
fn test_intersection_duplicates() {
    let a: HashSet<_> = vec![1, 1, 2, 2, 3, 3].into_iter().collect();
    let b: HashSet<_> = vec![2, 2, 3, 3, 4].into_iter().collect();
    let _set = &a & &b;
}

