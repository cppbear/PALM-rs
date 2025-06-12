// Answer 0

#[test]
fn test_sub_assign_case_1() {
    let mut a: HashSet<i32> = vec![1, 2, 3, 4, 5].into_iter().collect();
    let b: HashSet<i32> = vec![2, 3].into_iter().collect();
    a -= &b;
}

#[test]
fn test_sub_assign_case_2() {
    let mut a: HashSet<i32> = vec![1, 2, 3, 4, 5, 6, 7].into_iter().collect();
    let b: HashSet<i32> = vec![2, 3].into_iter().collect();
    a -= &b;
}

#[test]
fn test_sub_assign_case_3() {
    let mut a: HashSet<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8].into_iter().collect();
    let b: HashSet<i32> = vec![2, 3].into_iter().collect();
    a -= &b;
}

#[test]
fn test_sub_assign_edge_case_1() {
    let mut a: HashSet<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9].into_iter().collect();
    let b: HashSet<i32> = vec![2, 3].into_iter().collect();
    a -= &b;
}

#[test]
fn test_sub_assign_edge_case_2() {
    let mut a: HashSet<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9].into_iter().collect();
    let b: HashSet<i32> = vec![2, 3].into_iter().collect();
    a -= &b;
    // additional checks can be implemented if desired
}

