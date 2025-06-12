// Answer 0

#[derive(Debug)]
struct HeaderMap<T> {
    items: Vec<T>,
    max_size: usize,
}

#[derive(Debug)]
struct MaxSizeReached;

impl<T> HeaderMap<T> {
    fn new(max_size: usize) -> Self {
        Self {
            items: Vec::new(),
            max_size,
        }
    }

    fn try_insert2(&mut self, _key: &str, val: T) -> Result<Option<T>, MaxSizeReached> {
        if self.items.len() < self.max_size {
            self.items.push(val);
            Ok(None)
        } else {
            Err(MaxSizeReached)
        }
    }
}

fn try_insert<T>(
    self: &str,
    map: &mut HeaderMap<T>,
    val: T,
) -> Result<Option<T>, MaxSizeReached> {
    map.try_insert2(self, val)
}

#[test]
fn test_try_insert_success() {
    let mut map = HeaderMap::new(2);
    let result = try_insert("key1", &mut map, "value1");
    assert_eq!(result, Ok(None));
    assert_eq!(map.items.len(), 1);
}

#[test]
fn test_try_insert_reach_max_size() {
    let mut map = HeaderMap::new(1);
    let _ = try_insert("key1", &mut map, "value1");
    let result = try_insert("key2", &mut map, "value2");
    assert_eq!(result, Err(MaxSizeReached));
    assert_eq!(map.items.len(), 1);
}

#[test]
fn test_try_insert_with_multiple_success() {
    let mut map = HeaderMap::new(3);
    let result1 = try_insert("key1", &mut map, "value1");
    let result2 = try_insert("key2", &mut map, "value2");
    assert_eq!(result1, Ok(None));
    assert_eq!(result2, Ok(None));
    assert_eq!(map.items.len(), 2);
}

