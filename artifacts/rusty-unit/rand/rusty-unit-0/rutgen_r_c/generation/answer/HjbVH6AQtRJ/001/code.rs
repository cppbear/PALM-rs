// Answer 0

#[test]
fn test_random_iter_basic() {
    let result: Vec<i32> = rand::random_iter().take(5).collect();
    assert_eq!(result.len(), 5);
}

#[test]
fn test_random_iter_boundary() {
    let result: Vec<i32> = rand::random_iter().take(0).collect();
    assert_eq!(result.len(), 0);
}

#[test]
fn test_random_iter_large_take() {
    let result: Vec<i32> = rand::random_iter().take(1000).collect();
    assert_eq!(result.len(), 1000);
}

