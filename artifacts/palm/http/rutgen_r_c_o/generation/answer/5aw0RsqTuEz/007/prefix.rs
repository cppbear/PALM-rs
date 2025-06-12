// Answer 0

#[test]
fn test_try_reserve_zero() {
    let mut map = HeaderMap::with_capacity(10);
    map.try_reserve(0).unwrap();
}

#[test]
fn test_try_reserve_small() {
    let mut map = HeaderMap::with_capacity(10);
    map.try_reserve(5).unwrap();
}

#[test]
fn test_try_reserve_edge_case_raw_cap_equals_len() {
    let mut map = HeaderMap::with_capacity(0);
    map.try_reserve(1).unwrap(); // Initially empty, reserve for one, should be okay
    map.try_reserve(1).unwrap(); // This should be able to accommodate, as raw_cap == current len
}

#[test]
fn test_try_reserve_max_capacity() {
    let mut map = HeaderMap::try_with_capacity(32767).unwrap();
    map.try_reserve(1).unwrap(); // Should not exceed, remain within valid bounds
}

#[test]
fn test_try_reserve_full_capacity() {
    let mut map = HeaderMap::with_capacity(1 << 15);
    map.try_reserve(32767).unwrap(); // This should succeed as current capacity is more than enough
}

#[test]
fn test_try_reserve_exceed_capacity() {
    let mut map = HeaderMap::with_capacity(0);
    map.try_reserve(32768).unwrap_err(); // This should return an error due to exceeding max size
}

