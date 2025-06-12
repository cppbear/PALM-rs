// Answer 0


struct HeaderMap<T> {
    entries: Vec<T>,
    max_size: usize,
}

impl<T> HeaderMap<T> {
    fn new(max_size: usize) -> Self {
        HeaderMap {
            entries: Vec::new(),
            max_size,
        }
    }

    fn try_entry2(&mut self, entry: T) -> Result<Entry<'_, T>, MaxSizeReached> {
        if self.entries.len() >= self.max_size {
            return Err(MaxSizeReached);
        }
        self.entries.push(entry);
        Ok(Entry { value: &self.entries[self.entries.len() - 1] })
    }
}

struct Entry<'a, T> {
    value: &'a T,
}

struct MaxSizeReached;

#[test]
fn test_try_entry_success() {
    let mut map: HeaderMap<i32> = HeaderMap::new(3);
    let entry = 42;
    let result = map.try_entry(entry);
    assert!(result.is_ok());
}

#[test]
fn test_try_entry_max_size() {
    let mut map: HeaderMap<i32> = HeaderMap::new(2);
    map.try_entry(1).unwrap();
    map.try_entry(2).unwrap();
    let result = map.try_entry(3);
    assert!(result.is_err());
}

#[test]
#[should_panic]
fn test_try_entry_panic_condition() {
    let mut map: HeaderMap<String> = HeaderMap::new(0);
    let entry = "test".to_string();
    map.try_entry(entry).unwrap();
}


