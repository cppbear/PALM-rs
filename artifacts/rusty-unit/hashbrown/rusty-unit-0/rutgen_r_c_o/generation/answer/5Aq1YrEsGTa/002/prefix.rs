// Answer 0

use hashbrown::HashMap;

#[test]
fn test_retain_with_empty_map() {
    let mut map: HashMap<i32, i32> = HashMap::with_capacity_and_hasher_in(0, DefaultHashBuilder, Global);
    map.retain(|&k, _| k % 2 == 0);
}

#[test]
fn test_retain_with_even_capacity() {
    let mut map: HashMap<i32, i32> = HashMap::with_capacity_and_hasher_in(10, DefaultHashBuilder, Global);
    for x in 0..10 {
        map.insert(x, x * 10);
    }
    map.retain(|&k, _| k % 2 == 0);
}

#[test]
fn test_retain_with_odd_capacity() {
    let mut map: HashMap<i32, i32> = HashMap::with_capacity_and_hasher_in(5, DefaultHashBuilder, Global);
    map.insert(0, 0);
    map.insert(1, 10);
    map.insert(2, 20);
    map.insert(3, 30);
    map.insert(4, 40);
    map.retain(|&k, _| k % 2 == 0);
}

#[test]
fn test_retain_with_some_even_elements() {
    let mut map: HashMap<i32, i32> = HashMap::with_capacity_and_hasher_in(6, DefaultHashBuilder, Global);
    map.insert(0, 0);
    map.insert(1, 10);
    map.insert(2, 20);
    map.insert(3, 30);
    map.insert(4, 40);
    map.insert(5, 50);
    map.retain(|&k, _| k % 2 == 0);
}

#[test]
fn test_retain_with_all_odd_elements() {
    let mut map: HashMap<i32, i32> = HashMap::with_capacity_and_hasher_in(6, DefaultHashBuilder, Global);
    map.insert(1, 10);
    map.insert(3, 30);
    map.insert(5, 50);
    map.retain(|&k, _| k % 2 == 0);
}

#[test]
fn test_retain_with_all_even_elements() {
    let mut map: HashMap<i32, i32> = HashMap::with_capacity_and_hasher_in(6, DefaultHashBuilder, Global);
    map.insert(0, 0);
    map.insert(2, 20);
    map.insert(4, 40);
    map.retain(|&k, _| k % 2 == 0);
}

