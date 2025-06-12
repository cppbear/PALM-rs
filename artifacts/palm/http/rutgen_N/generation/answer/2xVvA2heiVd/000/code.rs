// Answer 0

#[derive(Debug)]
struct HeaderMap<T> {
    items: Vec<T>,
}

impl<T: PartialEq> HeaderMap<T> {
    fn find(&self, item: &T) -> Option<(usize, usize)> {
        for (index, elem) in self.items.iter().enumerate() {
            if elem == item {
                return Some((index, index));
            }
        }
        None
    }
}

struct TestStruct {
    value: i32,
}

#[test]
fn test_find_existing_item() {
    let header_map = HeaderMap {
        items: vec![TestStruct { value: 1 }, TestStruct { value: 2 }],
    };
    let test_item = TestStruct { value: 2 };
    let result = header_map.find(&test_item);
    assert_eq!(result, Some((1, 1)));
}

#[test]
fn test_find_non_existing_item() {
    let header_map = HeaderMap {
        items: vec![TestStruct { value: 1 }, TestStruct { value: 2 }],
    };
    let test_item = TestStruct { value: 3 };
    let result = header_map.find(&test_item);
    assert_eq!(result, None);
}

#[test]
fn test_find_empty_map() {
    let header_map: HeaderMap<TestStruct> = HeaderMap { items: vec![] };
    let test_item = TestStruct { value: 1 };
    let result = header_map.find(&test_item);
    assert_eq!(result, None);
}

