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

    fn try_insert2(&mut self, _key: usize, val: T) -> Result<Option<T>, MaxSizeReached> {
        if self.items.len() < self.max_size {
            self.items.push(val);
            Ok(None)
        } else {
            Err(MaxSizeReached)
        }
    }
}

#[derive(Debug)]
struct MaxSizeReached;

#[test]
fn test_try_insert_success() {
    let mut map = HeaderMap::new(2);
    let result = try_insert(0, &mut map, "value1");
    assert_eq!(result, Ok(None));
    assert_eq!(map.items.len(), 1);
}

#[test]
fn test_try_insert_overflow() {
    let mut map = HeaderMap::new(1);
    let _ = try_insert(0, &mut map, "value1");
    let result = try_insert(1, &mut map, "value2");
    assert_eq!(result, Err(MaxSizeReached));
    assert_eq!(map.items.len(), 1);
}

#[test]
fn test_try_insert_boundary() {
    let mut map = HeaderMap::new(2);
    let _ = try_insert(0, &mut map, "value1");
    let _ = try_insert(1, &mut map, "value2");
    let result = try_insert(2, &mut map, "value3");
    assert_eq!(result, Err(MaxSizeReached));
    assert_eq!(map.items.len(), 2);
}

