// Answer 0

#[test]
fn test_sub_assign_case1() {
    let mut a: HashSet<i32> = vec![1, 2, 3, 4].into_iter().collect();
    let b: HashSet<i32> = vec![3, 5].into_iter().collect();
    a -= &b;
}

#[test]
fn test_sub_assign_case2() {
    let mut a: HashSet<i32> = vec![10, 20, 30, 40].into_iter().collect();
    let b: HashSet<i32> = vec![20, 50].into_iter().collect();
    a -= &b;
}

#[test]
fn test_sub_assign_case3() {
    let mut a: HashSet<i32> = vec![5, 10, 15].into_iter().collect();
    let b: HashSet<i32> = vec![10].into_iter().collect();
    a -= &b;
}

#[test]
fn test_sub_assign_case4() {
    let mut a: HashSet<i32> = vec![100, 200, 300, 400, 500].into_iter().collect();
    let b: HashSet<i32> = vec![300, 700, 800].into_iter().collect();
    a -= &b;
}

#[test]
fn test_sub_assign_case5() {
    let mut a: HashSet<i32> = vec![7, 8, 9, 10].into_iter().collect();
    let b: HashSet<i32> = vec![10, 11].into_iter().collect();
    a -= &b;
}

