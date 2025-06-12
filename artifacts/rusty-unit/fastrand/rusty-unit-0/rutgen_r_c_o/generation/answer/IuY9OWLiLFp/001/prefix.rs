// Answer 0

#[test]
fn test_random_f64_in_range_0_to_1() {
    let result = f64();
}

#[test]
fn test_multiple_random_f64_in_range_0_to_1() {
    for _ in 0..100 {
        let _ = f64();
    }
}

#[test]
#[should_panic]
fn test_random_f64_should_not_panic() {
    let _ = f64();
}

