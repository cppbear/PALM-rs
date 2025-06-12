// Answer 0

#[test]
fn test_random_iter_integers() {
    let v: Vec<i32> = rand::random_iter().take(5).collect();
    assert_eq!(v.len(), 5);
}

#[test]
fn test_random_iter_floats() {
    let v: Vec<f64> = rand::random_iter().take(5).collect();
    assert_eq!(v.len(), 5);
}

