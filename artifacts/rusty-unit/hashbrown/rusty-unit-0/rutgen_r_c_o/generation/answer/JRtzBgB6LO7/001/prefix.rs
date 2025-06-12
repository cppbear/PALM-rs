// Answer 0

#[test]
fn test_extract_if_even_keys() {
    let mut map: HashMap<i32, i32> = HashMap::with_capacity_and_hasher_in(10, DefaultHashBuilder, Global);
    for i in 0..8 {
        map.insert(i, i);
    }
    let _extract = map.extract_if(|k, _v| k % 2 == 0);
}

#[test]
fn test_extract_if_odd_keys() {
    let mut map: HashMap<i32, i32> = HashMap::with_capacity_and_hasher_in(8, DefaultHashBuilder, Global);
    for i in 0..8 {
        map.insert(i, i);
    }
    let _extract = map.extract_if(|k, _v| k % 2 != 0);
}

#[test]
fn test_extract_if_no_elements() {
    let mut map: HashMap<i32, i32> = HashMap::with_capacity_and_hasher_in(0, DefaultHashBuilder, Global);
    let _extract = map.extract_if(|_k, _v| true);
}

#[test]
fn test_extract_if_full_capacity() {
    let mut map: HashMap<i32, i32> = HashMap::with_capacity_and_hasher_in(10_000, DefaultHashBuilder, Global);
    for i in 0..10_000 {
        map.insert(i, i);
    }
    let _extract = map.extract_if(|k, _v| k < 5000);
}

#[test]
fn test_extract_if_exceed_map_capacity() {
    let mut map: HashMap<i32, i32> = HashMap::with_capacity_and_hasher_in(1_000_000, DefaultHashBuilder, Global);
    for i in 0..1_000_000 {
        map.insert(i, i);
    }
    let _extract = map.extract_if(|k, _v| k % 3 == 0);
}

#[test]
fn test_extract_if_high_key_value() {
    let mut map: HashMap<u64, u64> = HashMap::with_capacity_and_hasher_in(100, DefaultHashBuilder, Global);
    for i in 0..100 {
        map.insert((1 << 32) + i as u64, (1 << 32) + i as u64);
    }
    let _extract = map.extract_if(|_k, v| *v > 1 << 32);
}

#[test]
fn test_extract_if_large_iter() {
    let mut map: HashMap<i32, i32> = HashMap::with_capacity_and_hasher_in(100_000, DefaultHashBuilder, Global);
    for i in 0..100_000 {
        map.insert(i, i);
    }
    let _extract = map.extract_if(|k, _v| k % 2 == 0);
}

