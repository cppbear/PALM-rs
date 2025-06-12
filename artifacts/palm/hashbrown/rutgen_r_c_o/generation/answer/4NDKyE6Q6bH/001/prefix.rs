// Answer 0

#[test]
fn test_allocation_size_empty() {
    let map: HashMap<i32, i32> = HashMap::default();
    let _ = map.allocation_size();
}

#[test]
fn test_allocation_size_single_entry() {
    let mut map: HashMap<i32, i32> = HashMap::default();
    map.insert(1, 10);
    let _ = map.allocation_size();
}

#[test]
fn test_allocation_size_multiple_entries() {
    let mut map: HashMap<i32, i32> = HashMap::default();
    for i in 0..1000 {
        map.insert(i, i * 10);
    }
    let _ = map.allocation_size();
}

#[test]
fn test_allocation_size_large_number_of_entries() {
    let mut map: HashMap<i32, i32> = HashMap::default();
    for i in 0..10_000 {
        map.insert(i, i * 100);
    }
    let _ = map.allocation_size();
}

#[test]
fn test_allocation_size_max_entries() {
    let mut map: HashMap<i32, i32> = HashMap::default();
    for i in 0..1_000_000 {
        map.insert(i, i * 1000);
    }
    let _ = map.allocation_size();
}

#[test]
fn test_allocation_size_edge_case() {
    let mut map: HashMap<i32, i32> = HashMap::default();
    for i in 0..4_294_967_295 {
        map.insert(i, i);
    }
    let _ = map.allocation_size();
}

