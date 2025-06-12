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
        HeaderMap {
            items: Vec::new(),
            max_size,
        }
    }

    fn try_insert2(&mut self, _key: usize, val: T) -> Result<Option<T>, MaxSizeReached> {
        if self.items.len() >= self.max_size {
            Err(MaxSizeReached)
        } else {
            self.items.push(val);
            Ok(None)
        }
    }
}

fn try_insert<T>(
    self_key: usize,
    map: &mut HeaderMap<T>,
    val: T,
) -> Result<Option<T>, MaxSizeReached> {
    map.try_insert2(self_key, val)
}

#[test]
fn test_try_insert_success() {
    let mut map = HeaderMap::<i32>::new(2);
    let result = try_insert(0, &mut map, 42);
    assert_eq!(result, Ok(None));
    assert_eq!(map.items.len(), 1);
}

#[test]
fn test_try_insert_over_max_size() {
    let mut map = HeaderMap::<i32>::new(1);
    let _ = try_insert(0, &mut map, 42);
    let result = try_insert(1, &mut map, 100);
    assert_eq!(result, Err(MaxSizeReached));
    assert_eq!(map.items.len(), 1);
}

#[test]
fn test_try_insert_boundaries() {
    let mut map = HeaderMap::<i32>::new(2);
    let result1 = try_insert(0, &mut map, 1);
    assert_eq!(result1, Ok(None));
    
    let result2 = try_insert(1, &mut map, 2);
    assert_eq!(result2, Ok(None));
    
    let result3 = try_insert(2, &mut map, 3);
    assert_eq!(result3, Err(MaxSizeReached));
    
    assert_eq!(map.items.len(), 2);
}

