// Answer 0

#[test]
fn test_hashmap_new_default() {
    use crate::HashMap;

    let map: HashMap<&str, i32> = HashMap::new();
    assert_eq!(map.len(), 0);
    assert_eq!(map.capacity(), 0);
}

#[test]
fn test_hashmap_new_with_capacity() {
    use crate::HashMap;

    let map: HashMap<&str, i32> = HashMap::with_capacity(0);
    assert_eq!(map.len(), 0);
    assert_eq!(map.capacity(), 0);
}

