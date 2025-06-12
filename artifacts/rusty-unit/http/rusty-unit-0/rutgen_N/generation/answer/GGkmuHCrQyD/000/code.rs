// Answer 0

#[derive(Debug)]
struct HeaderMap<T> {
    entries: Vec<T>,
}

#[derive(Debug)]
struct Entry<'a, T> {
    value: &'a T,
}

#[derive(Debug)]
struct MaxSizeReached;

impl<T> HeaderMap<T> {
    fn try_entry2(&mut self, _key: T) -> Result<Entry<T>, MaxSizeReached> {
        if self.entries.len() < 10 { // arbitrary max size for testing
            self.entries.push(_key);
            Ok(Entry { value: &self.entries[self.entries.len() - 1] })
        } else {
            Err(MaxSizeReached)
        }
    }
}

fn try_entry<T>(self, map: &mut HeaderMap<T>) -> Result<Entry<'_, T>, MaxSizeReached> {
    map.try_entry2(self)
}

#[test]
fn test_try_entry_success() {
    let mut header_map = HeaderMap { entries: Vec::new() };
    let result: Result<Entry<T>, MaxSizeReached> = try_entry(1, &mut header_map); // using an integer as T
    assert!(result.is_ok());
    assert_eq!(header_map.entries.len(), 1);
}

#[test]
fn test_try_entry_max_size_reached() {
    let mut header_map = HeaderMap { entries: (0..10).collect::<Vec<_>>() }; // pre-fill to max size
    let result: Result<Entry<i32>, MaxSizeReached> = try_entry(11, &mut header_map);
    assert!(result.is_err());
    assert_eq!(header_map.entries.len(), 10);
}

