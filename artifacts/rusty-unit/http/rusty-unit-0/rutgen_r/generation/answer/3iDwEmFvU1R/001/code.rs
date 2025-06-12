// Answer 0

#[derive(Debug)]
struct HeaderMap<T> {
    values: Vec<T>,
    max_size: usize,
}

impl<T> HeaderMap<T> {
    fn new(max_size: usize) -> Self {
        Self {
            values: Vec::new(),
            max_size,
        }
    }

    fn try_append2(&mut self, _: T, val: T) -> Result<bool, MaxSizeReached> {
        if self.values.len() >= self.max_size {
            Err(MaxSizeReached)
        } else {
            self.values.push(val);
            Ok(true)
        }
    }
}

#[derive(Debug)]
struct MaxSizeReached;

#[test]
fn test_try_append_success() {
    let mut header_map = HeaderMap::new(2);
    let result = try_append((), &mut header_map, "value1");
    assert_eq!(result, Ok(true));
    assert_eq!(header_map.values.len(), 1);
}

#[test]
fn test_try_append_reach_max_size() {
    let mut header_map = HeaderMap::new(1);
    let _ = try_append((), &mut header_map, "value1");
    let result = try_append((), &mut header_map, "value2");
    assert_eq!(result, Err(MaxSizeReached));
}

#[test]
fn test_try_append_no_space_left() {
    let mut header_map = HeaderMap::new(0);
    let result = try_append((), &mut header_map, "value1");
    assert_eq!(result, Err(MaxSizeReached));
}

#[test]
fn test_try_append_with_different_types() {
    let mut header_map = HeaderMap::new(2);
    let result1 = try_append((), &mut header_map, 42);
    let result2 = try_append((), &mut header_map, 3.14);
    assert_eq!(result1, Ok(true));
    assert_eq!(result2, Ok(true));
    assert_eq!(header_map.values.len(), 2);
}

