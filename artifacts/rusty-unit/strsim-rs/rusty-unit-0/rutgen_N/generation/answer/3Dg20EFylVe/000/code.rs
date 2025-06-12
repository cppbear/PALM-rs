// Answer 0

#[derive(Default)]
struct ValueType {
    value: u32,
}

struct TestStruct {
    map: Option<Vec<Entry>>,
    lookup: fn(u32) -> usize,
}

struct Entry {
    value: ValueType,
}

impl TestStruct {
    fn get(&self, key: u32) -> ValueType {
        self.map
            .as_ref()
            .map_or_else(|| Default::default(), |map| map[self.lookup(key)].value)
    }
}

#[test]
fn test_get_with_empty_map() {
    let test_struct = TestStruct {
        map: None,
        lookup: |_: u32| 0,
    };
    let result = test_struct.get(1);
    assert_eq!(result.value, 0);
}

#[test]
fn test_get_with_valid_key() {
    let entry = Entry { value: ValueType { value: 42 } };
    let map = vec![entry];
    let test_struct = TestStruct {
        map: Some(map),
        lookup: |key| key as usize,
    };
    let result = test_struct.get(0);
    assert_eq!(result.value, 42);
}

#[test]
fn test_get_with_out_of_bound_key() {
    let entry = Entry { value: ValueType { value: 100 } };
    let map = vec![entry];
    let test_struct = TestStruct {
        map: Some(map),
        lookup: |key| key as usize,
    };
    let result = test_struct.get(1);
    assert_eq!(result.value, 0);
}

