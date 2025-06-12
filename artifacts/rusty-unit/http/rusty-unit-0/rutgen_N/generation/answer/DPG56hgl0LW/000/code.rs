// Answer 0

#[derive(Debug)]
struct HeaderMap<T> {
    items: Vec<T>,
    max_size: usize,
}

impl<T> HeaderMap<T> {
    fn new(max_size: usize) -> Self {
        HeaderMap {
            items: Vec::new(),
            max_size,
        }
    }

    fn try_append2(&mut self, _header: &Header, val: T) -> Result<bool, MaxSizeReached> {
        if self.items.len() < self.max_size {
            self.items.push(val);
            Ok(true)
        } else {
            Err(MaxSizeReached)
        }
    }
}

#[derive(Debug)]
struct Header;

#[derive(Debug)]
struct MaxSizeReached;

impl Header {
    fn try_append<T>(self, map: &mut HeaderMap<T>, val: T) -> Result<bool, MaxSizeReached> {
        map.try_append2(&self, val)
    }
}

#[test]
fn test_try_append_success() {
    let header = Header;
    let mut map = HeaderMap::new(2);
    let result = header.try_append(&mut map, "value1");
    assert_eq!(result, Ok(true));
    assert_eq!(map.items.len(), 1);
}

#[test]
fn test_try_append_failure() {
    let header = Header;
    let mut map = HeaderMap::new(1);
    let _ = header.try_append(&mut map, "value1");
    let result = header.try_append(&mut map, "value2");
    assert_eq!(result, Err(MaxSizeReached));
    assert_eq!(map.items.len(), 1);
}

