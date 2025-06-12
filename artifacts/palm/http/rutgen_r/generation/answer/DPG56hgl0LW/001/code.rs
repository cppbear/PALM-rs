// Answer 0

#[derive(Debug)]
struct MockHeaderMap<T> {
    entries: Vec<T>,
    max_size: usize,
}

#[derive(Debug)]
struct MaxSizeReached;

impl<T> MockHeaderMap<T> {
    fn new(max_size: usize) -> Self {
        MockHeaderMap {
            entries: Vec::new(),
            max_size,
        }
    }

    fn try_append2(&mut self, _: &str, val: T) -> Result<bool, MaxSizeReached> {
        if self.entries.len() >= self.max_size {
            Err(MaxSizeReached)
        } else {
            self.entries.push(val);
            Ok(true)
        }
    }
}

#[test]
fn test_try_append_success() {
    let mut map = MockHeaderMap::new(2);
    let result = try_append("HeaderName", &mut map, "Value1");
    assert_eq!(result, Ok(true));
    let result2 = try_append("HeaderName", &mut map, "Value2");
    assert_eq!(result2, Ok(true));
}

#[test]
fn test_try_append_max_size_reached() {
    let mut map = MockHeaderMap::new(2);
    let _ = try_append("HeaderName", &mut map, "Value1");
    let _ = try_append("HeaderName", &mut map, "Value2");
    let result = try_append("HeaderName", &mut map, "Value3");
    assert_eq!(result, Err(MaxSizeReached));
}

