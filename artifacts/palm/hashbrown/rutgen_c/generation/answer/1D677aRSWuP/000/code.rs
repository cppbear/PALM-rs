// Answer 0

#[test]
fn test_with_hasher_creates_empty_hashmap() {
    use crate::{HashMap, DefaultHashBuilder};

    let builder = DefaultHashBuilder::default();
    let map: HashMap<i32, i32, DefaultHashBuilder> = HashMap::with_hasher(builder);

    assert_eq!(map.table.len(), 0);
    assert_eq!(map.table.capacity(), 0);
}

#[test]
fn test_with_hasher_allows_insertion() {
    use crate::{HashMap, DefaultHashBuilder};

    let builder = DefaultHashBuilder::default();
    let mut map: HashMap<i32, i32, DefaultHashBuilder> = HashMap::with_hasher(builder);

    assert_eq!(map.table.len(), 0);
    assert_eq!(map.table.capacity(), 0);

    map.table.insert(1, 2);
    assert_eq!(map.table.len(), 1);
}

