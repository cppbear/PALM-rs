// Answer 0

#[test]
fn test_reserve_positive_additional() {
    let mut map = HeaderMap::with_capacity(10);
    map.reserve(1);
}

#[test]
fn test_reserve_multiple_additional() {
    let mut map = HeaderMap::with_capacity(10);
    map.reserve(5);
}

#[test]
fn test_reserve_exceeding_current_capacity() {
    let mut map = HeaderMap::with_capacity(4);
    map.reserve(10);
}

#[test]
fn test_reserve_edge_case_max_size() {
    let mut map = HeaderMap::with_capacity(1);
    map.reserve(32767);
}

#[should_panic(expected = "size overflows MAX_SIZE")]
#[test]
fn test_reserve_exceed_max_size() {
    let mut map = HeaderMap::with_capacity(32767);
    map.reserve(1);
}

