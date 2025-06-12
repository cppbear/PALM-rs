// Answer 0

#[test]
fn test_random_iter_i32() {
    let random_numbers: Vec<i32> = rand::random_iter().take(5).collect();
    assert_eq!(random_numbers.len(), 5);
}

#[test]
fn test_random_iter_f64() {
    let random_numbers: Vec<f64> = rand::random_iter().take(5).collect();
    assert_eq!(random_numbers.len(), 5);
}

#[test]
fn test_random_iter_empty() {
    let random_numbers: Vec<i32> = rand::random_iter().take(0).collect();
    assert_eq!(random_numbers.len(), 0);
}

