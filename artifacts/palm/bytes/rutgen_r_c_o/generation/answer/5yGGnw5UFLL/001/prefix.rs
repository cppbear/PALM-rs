// Answer 0

#[test]
fn test_set_limit_zero() {
    let mut take = Take { inner: vec![b'a'; 10], limit: 10 };
    take.set_limit(0);
}

#[test]
fn test_set_limit_ten() {
    let mut take = Take { inner: vec![b'b'; 10], limit: 5 };
    take.set_limit(10);
}

#[test]
fn test_set_limit_edge_case_one() {
    let mut take = Take { inner: vec![b'c'; 10], limit: 10 };
    take.set_limit(1);
}

#[test]
fn test_set_limit_edge_case_two() {
    let mut take = Take { inner: vec![b'd'; 10], limit: 0 };
    take.set_limit(1);
}

#[test]
fn test_set_limit_no_change() {
    let mut take = Take { inner: vec![b'e'; 10], limit: 7 };
    take.set_limit(7);
}

