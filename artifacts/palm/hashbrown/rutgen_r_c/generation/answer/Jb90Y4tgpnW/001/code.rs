// Answer 0

#[test]
fn test_get_many_unchecked_mut_unique_keys() {
    use crate::{HashMap, DefaultHashBuilder, Hash};

    let mut map = HashMap::new();
    map.insert("key1".to_string(), 1);
    map.insert("key2".to_string(), 2);

    // SAFETY: Unique keys
    let result: [Option<&mut i32>; 2] = unsafe { map.get_many_unchecked_mut(["key1", "key2"]) };
    assert_eq!(result, [Some(&mut 1), Some(&mut 2)]);
}

#[test]
fn test_get_many_unchecked_mut_missing_key() {
    use crate::{HashMap, DefaultHashBuilder, Hash};

    let mut map = HashMap::new();
    map.insert("key1".to_string(), 1);

    // SAFETY: One existing key and one missing key
    let result: [Option<&mut i32>; 2] = unsafe { map.get_many_unchecked_mut(["key1", "key2"]) };
    assert_eq!(result, [Some(&mut 1), None]);
}

#[test]
fn test_get_many_unchecked_mut_non_overlapping_keys() {
    use crate::{HashMap, DefaultHashBuilder, Hash};

    let mut map = HashMap::new();
    map.insert("key1".to_string(), 10);
    map.insert("key2".to_string(), 20);
    map.insert("key3".to_string(), 30);

    // SAFETY: Non-overlapping keys
    let result: [Option<&mut i32>; 3] = unsafe { map.get_many_unchecked_mut(["key1", "key2", "key3"]) };
    assert_eq!(result, [Some(&mut 10), Some(&mut 20), Some(&mut 30)]);
}

#[should_panic]
#[test]
fn test_get_many_unchecked_mut_overlapping_keys() {
    use crate::{HashMap, DefaultHashBuilder, Hash};

    let mut map = HashMap::new();
    map.insert("key1".to_string(), 100);
    map.insert("key2".to_string(), 200);

    // SAFETY: Overlapping keys (this should panic)
    let _result: [Option<&mut i32>; 2] = unsafe { map.get_many_unchecked_mut(["key1", "key1"]) };
}

