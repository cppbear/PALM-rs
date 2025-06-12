// Answer 0

#[test]
fn test_reserve_zero() {
    let mut map: HashMap<&str, i32> = HashMap::new();
    map.reserve(0);
}

#[test]
fn test_reserve_small_value() {
    let mut map: HashMap<&str, i32> = HashMap::new();
    map.reserve(1);
}

#[test]
fn test_reserve_large_value() {
    let mut map: HashMap<&str, i32> = HashMap::new();
    map.reserve(100);
}

#[test]
#[should_panic]
fn test_reserve_exceed_isize_max() {
    let mut map: HashMap<&str, i32> = HashMap::new();
    map.reserve(isize::MAX as usize + 1);
}

