// Answer 0

#[test]
fn test_advance_zero_delta() {
    let mut rng = Lcg64Xsh32::new(12345, 1);
    rng.advance(0);
}

#[test]
fn test_advance_large_delta() {
    let mut rng = Lcg64Xsh32::new(67890, 2);
    rng.advance(u64::MAX);
}

#[test]
fn test_advance_multiple_calls() {
    let mut rng = Lcg64Xsh32::new(11111, 3);
    rng.advance(1);
    rng.advance(1);
    rng.advance(1);
} 

#[test]
fn test_advance_random_delta() {
    let mut rng = Lcg64Xsh32::new(22222, 4);
    rng.advance(5);
}

#[test]
fn test_advance_small_delta() {
    let mut rng = Lcg64Xsh32::new(33333, 5);
    rng.advance(2);
}

