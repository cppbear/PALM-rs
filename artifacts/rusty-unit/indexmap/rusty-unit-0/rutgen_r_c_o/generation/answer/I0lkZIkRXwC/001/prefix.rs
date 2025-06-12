// Answer 0

#[test]
fn test_pop_empty() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::new();
    let result = map.pop();
}

#[test]
fn test_pop_single_entry() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::new();
    map.insert(1, 10);
    let result = map.pop();
}

#[test]
fn test_pop_multiple_entries() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::new();
    map.insert(1, 10);
    map.insert(2, 20);
    let result = map.pop();
}

#[test]
fn test_pop_after_single_removal() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::new();
    map.insert(1, 10);
    map.pop(); // Remove first entry
    let result = map.pop(); // Now should pop the second (last) entry
}

#[test]
fn test_pop_on_two_entries() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::new();
    map.insert(1, 10); 
    map.insert(2, 20); 
    let _ = map.pop(); // Removes the last added entry
    let result = map.pop(); // Now should remove the remaining entry
}

