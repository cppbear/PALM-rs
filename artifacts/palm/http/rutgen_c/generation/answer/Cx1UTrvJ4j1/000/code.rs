// Answer 0

#[test]
fn test_reserve_increases_capacity() {
    struct TestHeaderValue;
    
    let mut map: HeaderMap<TestHeaderValue> = HeaderMap::with_capacity(0);
    
    let initial_capacity = map.capacity();
    map.reserve(10);
    
    assert!(map.capacity() > initial_capacity);
}

#[test]
#[should_panic(expected = "size overflows MAX_SIZE")]
fn test_reserve_panics_on_exceeding_max_size() {
    struct TestHeaderValue;

    let mut map: HeaderMap<TestHeaderValue> = HeaderMap::with_capacity(MAX_SIZE);
    
    // Attempt to reserve more than what can be accommodated
    map.reserve(10);
} 

#[test]
fn test_reserve_no_effect_when_zero() {
    struct TestHeaderValue;

    let mut map: HeaderMap<TestHeaderValue> = HeaderMap::with_capacity(0);
    
    let initial_capacity = map.capacity();
    map.reserve(0);
    
    assert_eq!(map.capacity(), initial_capacity);
}

#[test]
fn test_reserve_multiple_calls() {
    struct TestHeaderValue;

    let mut map: HeaderMap<TestHeaderValue> = HeaderMap::with_capacity(0);
    
    map.reserve(5);
    let capacity_after_first_reserve = map.capacity();
    
    map.reserve(3);
    
    assert!(map.capacity() >= capacity_after_first_reserve);
}

