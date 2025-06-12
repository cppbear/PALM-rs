// Answer 0

#[test]
fn test_retain_all_elements() {
    let mut map: HashMap<i32, i32> = HashMap::with_capacity_and_hasher_in(10, DefaultHashBuilder, Global);
    for x in 0..10 {
        map.insert(x, x * 10);
    }
    map.retain(|&k, _| true);
}

#[test]
fn test_retain_no_elements() {
    let mut map: HashMap<i32, i32> = HashMap::with_capacity_and_hasher_in(10, DefaultHashBuilder, Global);
    for x in 0..10 {
        map.insert(x, x * 10);
    }
    map.retain(|&k, _| false);
}

#[test]
fn test_retain_half_elements() {
    let mut map: HashMap<i32, i32> = HashMap::with_capacity_and_hasher_in(10, DefaultHashBuilder, Global);
    for x in 0..10 {
        map.insert(x, x * 10);
    }
    map.retain(|&k, _| k % 2 == 0);
}

#[test]
fn test_retain_empty_map() {
    let mut map: HashMap<i32, i32> = HashMap::with_capacity_and_hasher_in(0, DefaultHashBuilder, Global);
    map.retain(|&k, _| k % 2 == 0);
}

#[test]
fn test_retain_with_random_elements() {
    let mut map: HashMap<i32, i32> = HashMap::with_capacity_and_hasher_in(20, DefaultHashBuilder, Global);
    for x in 0..20 {
        map.insert(x, x * 10);
    }
    map.retain(|&k, _| k % 3 == 0);
}

