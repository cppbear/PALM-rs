// Answer 0

#[test]
fn test_try_reserve_zero_capacity() {
    let mut map: HashMap<&str, isize> = HashMap::new();
    let _ = map.try_reserve(0);
}

#[test]
fn test_try_reserve_some_capacity() {
    let mut map: HashMap<&str, isize> = HashMap::new();
    let _ = map.try_reserve(10);
}

#[test]
#[should_panic]
fn test_try_reserve_capacity_overflow() {
    let mut map: HashMap<i32, i32> = HashMap::new();
    let _ = map.try_reserve(usize::MAX);
}

