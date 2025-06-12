// Answer 0

#[derive(Debug)]
struct HeaderMap<T> {
    values: Vec<T>,
    max_size: usize,
}

#[derive(Debug)]
struct MaxSizeReached;

impl<T> HeaderMap<T> {
    fn new(max_size: usize) -> Self {
        HeaderMap {
            values: Vec::new(),
            max_size,
        }
    }

    fn try_append2(&mut self, _header: Header, val: T) -> Result<bool, MaxSizeReached> {
        if self.values.len() < self.max_size {
            self.values.push(val);
            Ok(true)
        } else {
            Err(MaxSizeReached)
        }
    }
}

#[derive(Debug)]
struct Header;

impl Header {
    fn try_append<T>(self, map: &mut HeaderMap<T>, val: T) -> Result<bool, MaxSizeReached> {
        map.try_append2(self, val)
    }
}

#[test]
fn test_try_append_success() {
    let header = Header;
    let mut map = HeaderMap::new(2);
    let result = header.try_append(&mut map, "Value1");
    assert_eq!(result, Ok(true));
    assert_eq!(map.values.len(), 1);
}

#[test]
fn test_try_append_reaches_max_size() {
    let header = Header;
    let mut map = HeaderMap::new(2);
    let _ = header.try_append(&mut map, "Value1").unwrap();
    let _ = header.try_append(&mut map, "Value2").unwrap();
    let result = header.try_append(&mut map, "Value3");
    assert_eq!(result, Err(MaxSizeReached));
    assert_eq!(map.values.len(), 2);
}

#[test]
fn test_try_append_empty_map() {
    let header = Header;
    let mut map = HeaderMap::new(1);
    let result = header.try_append(&mut map, "FirstValue");
    assert_eq!(result, Ok(true));
    assert_eq!(map.values.len(), 1);
}

