// Answer 0

#[derive(Debug)]
struct HeaderMap<T> {
    data: Vec<T>,
}

impl<T: PartialEq> HeaderMap<T> {
    fn new(data: Vec<T>) -> Self {
        HeaderMap { data }
    }

    fn find(&self, target: T) -> Option<(usize, usize)> {
        for (i, item) in self.data.iter().enumerate() {
            if *item == target {
                return Some((i, 1)); // returning (index, some_value)
            }
        }
        None
    }
}

struct Header {
    value: usize,
}

impl PartialEq for Header {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

#[test]
fn test_find_existing_header() {
    let header = Header { value: 1 };
    let map = HeaderMap::new(vec![Header { value: 1 }, Header { value: 2 }]);
    let result = find(&header, &map);
    assert_eq!(result, Some((0, 1)));
}

#[test]
fn test_find_non_existing_header() {
    let header = Header { value: 3 };
    let map = HeaderMap::new(vec![Header { value: 1 }, Header { value: 2 }]);
    let result = find(&header, &map);
    assert_eq!(result, None);
}

#[test]
fn test_find_boundary_condition_empty_map() {
    let header = Header { value: 1 };
    let map: HeaderMap<Header> = HeaderMap::new(vec![]);
    let result = find(&header, &map);
    assert_eq!(result, None);
}

