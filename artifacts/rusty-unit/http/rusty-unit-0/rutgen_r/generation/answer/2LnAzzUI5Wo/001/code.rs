// Answer 0

#[derive(Debug, PartialEq)]
struct HeaderMap<T> {
    data: Vec<T>,
}

impl<T: PartialEq> HeaderMap<T> {
    fn find(&self, item: T) -> Option<(usize, usize)> {
        for (index, value) in self.data.iter().enumerate() {
            if *value == item {
                return Some((index, index)); // returning both indices as the same for this use case
            }
        }
        None
    }
}

#[derive(Debug, PartialEq)]
struct Header {
    value: String,
}

impl Header {
    fn new(value: &str) -> Self {
        Header {
            value: value.to_string(),
        }
    }
}

impl PartialEq for Header {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

fn find<T>(&self, map: &HeaderMap<T>) -> Option<(usize, usize)> {
    map.find(self.clone())
}

#[test]
fn test_find_existing_item() {
    let header = Header::new("Content-Type");
    let map = HeaderMap {
        data: vec![Header::new("Content-Type"), Header::new("Accept")],
    };
    
    let result = find(&header, &map);
    assert_eq!(result, Some((0, 0)));
}

#[test]
fn test_find_non_existing_item() {
    let header = Header::new("Authorization");
    let map = HeaderMap {
        data: vec![Header::new("Content-Type"), Header::new("Accept")],
    };
    
    let result = find(&header, &map);
    assert_eq!(result, None);
}

#[test]
fn test_find_empty_map() {
    let header = Header::new("Content-Type");
    let map = HeaderMap {
        data: Vec::new(),
    };
    
    let result = find(&header, &map);
    assert_eq!(result, None);
}

#[test]
fn test_find_multiple_identical_items() {
    let header = Header::new("Duplicate");
    let map = HeaderMap {
        data: vec![Header::new("Duplicate"), Header::new("Duplicate"), Header::new("Other")],
    };
    
    let result = find(&header, &map);
    assert_eq!(result, Some((0, 0)));
}

#[test]
fn test_find_middle_item() {
    let header = Header::new("Middle");
    let map = HeaderMap {
        data: vec![Header::new("Start"), Header::new("Middle"), Header::new("End")],
    };
    
    let result = find(&header, &map);
    assert_eq!(result, Some((1, 1)));
}

