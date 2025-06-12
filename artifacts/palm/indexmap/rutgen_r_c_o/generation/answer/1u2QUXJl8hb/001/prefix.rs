// Answer 0

#[test]
fn test_reverse_empty_map() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::with_capacity(0);
    map.reverse();
}

#[test]
fn test_reverse_single_element_map() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::with_capacity(1);
    map.insert(1, 100);
    map.reverse();
}

#[test]
fn test_reverse_two_elements_map() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::with_capacity(2);
    map.insert(1, 100);
    map.insert(2, 200);
    map.reverse();
}

#[test]
fn test_reverse_multiple_elements_map() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::with_capacity(5);
    map.insert(1, 100);
    map.insert(2, 200);
    map.insert(3, 300);
    map.insert(4, 400);
    map.insert(5, 500);
    map.reverse();
}

#[test]
fn test_reverse_large_map() {
    let mut map: IndexMap<usize, usize, RandomState> = IndexMap::with_capacity(1000);
    for i in 0..1000 {
        map.insert(i, i * 10);
    }
    map.reverse();
}

#[test]
#[should_panic]
fn test_reverse_panic_out_of_bounds() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::with_capacity(1);
    map.insert(1, 100);
    map.delete(1); // Simulate out-of-bounds access
    map.reverse();
}

