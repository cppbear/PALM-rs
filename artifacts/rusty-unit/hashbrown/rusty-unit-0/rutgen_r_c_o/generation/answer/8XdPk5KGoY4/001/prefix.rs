// Answer 0

#[test]
fn test_with_capacity_in_zero_capacity() {
    let bump = Bump::new();
    let map = HashMap::with_capacity_in(0, &bump);
}

#[test]
fn test_with_capacity_in_small_capacity() {
    let bump = Bump::new();
    let map = HashMap::with_capacity_in(1, &bump);
}

#[test]
fn test_with_capacity_in_mid_capacity() {
    let bump = Bump::new();
    let map = HashMap::with_capacity_in(100, &bump);
}

#[test]
fn test_with_capacity_in_large_capacity() {
    let bump = Bump::new();
    let map = HashMap::with_capacity_in(1000, &bump);
}

#[test]
fn test_with_capacity_in_max_capacity() {
    let bump = Bump::new();
    let map = HashMap::with_capacity_in(1_000_000, &bump);
}

