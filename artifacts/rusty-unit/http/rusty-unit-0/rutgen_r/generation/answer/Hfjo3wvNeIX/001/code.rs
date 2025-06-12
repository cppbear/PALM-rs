// Answer 0

#[derive(Debug)]
struct HeaderMap<T> {
    data: Vec<T>,
    max_size: usize,
}

impl<T> HeaderMap<T> {
    fn new(max_size: usize) -> Self {
        HeaderMap {
            data: Vec::new(),
            max_size,
        }
    }

    fn try_insert2(&mut self, _: &str, val: T) -> Result<Option<T>, MaxSizeReached> {
        if self.data.len() < self.max_size {
            self.data.push(val);
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
    let result = try_insert("key1", &mut map, "value1");
    assert_eq!(result, Ok(None));
    assert_eq!(map.data.len(), 1);
}

#[test]
fn test_try_insert_overflow() {
    let mut map = HeaderMap::new(1);
    let _ = try_insert("key1", &mut map, "value1");
    let result = try_insert("key2", &mut map, "value2");
    assert_eq!(result, Err(MaxSizeReached));
}

#[test]
fn test_try_insert_boundary_condition() {
    let mut map = HeaderMap::new(0);
    let result = try_insert("key1", &mut map, "value1");
    assert_eq!(result, Err(MaxSizeReached));
}

