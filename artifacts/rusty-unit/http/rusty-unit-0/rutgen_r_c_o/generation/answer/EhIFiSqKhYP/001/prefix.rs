// Answer 0

#[test]
fn test_with_capacity_zero() {
    let map: HeaderMap<u32> = HeaderMap::with_capacity(0);
}

#[test]
fn test_with_capacity_minimal() {
    let map: HeaderMap<u32> = HeaderMap::with_capacity(1);
}

#[test]
fn test_with_capacity_small() {
    let map: HeaderMap<u32> = HeaderMap::with_capacity(10);
}

#[test]
fn test_with_capacity_exact_power_of_two() {
    let map: HeaderMap<u32> = HeaderMap::with_capacity(8);
}

#[test]
fn test_with_capacity_large() {
    let map: HeaderMap<u32> = HeaderMap::with_capacity(16384);
}

#[should_panic]
fn test_with_capacity_exceeds_max_size() {
    let _map: HeaderMap<u32> = HeaderMap::with_capacity(32768);
}

#[test]
fn test_with_capacity_near_max_size() {
    let map: HeaderMap<u32> = HeaderMap::with_capacity(32767);
}

