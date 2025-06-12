// Answer 0

#[test]
fn test_random_iter_with_i32() {
    let v: Vec<i32> = rand::random_iter().take(5).collect();
    assert_eq!(v.len(), 5);
}

#[test]
fn test_random_iter_with_f64() {
    let v: Vec<f64> = rand::random_iter().take(5).collect();
    assert_eq!(v.len(), 5);
}

#[test]
fn test_random_iter_take_zero() {
    let v: Vec<i32> = rand::random_iter().take(0).collect();
    assert_eq!(v.len(), 0);
}

#[test]
#[should_panic]
fn test_random_iter_panic_on_exceeding_cap() {
    let large_value: Result<Vec<i32>, _> = std::panic::catch_unwind(|| {
        let v: Vec<i32> = rand::random_iter().take(usize::MAX).collect();
        v
    });
    assert!(large_value.is_err());
}

