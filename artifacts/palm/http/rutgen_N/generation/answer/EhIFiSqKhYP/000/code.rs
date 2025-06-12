// Answer 0

#[derive(Debug)]
struct HeaderMap<T> {
    capacity: usize,
    elements: Vec<T>,
}

impl<T> HeaderMap<T> {
    const MAX_SIZE: usize = 100;

    pub fn with_capacity(capacity: usize) -> HeaderMap<T> {
        Self::try_with_capacity(capacity).expect("size overflows MAX_SIZE")
    }

    pub fn try_with_capacity(capacity: usize) -> Result<HeaderMap<T>, &'static str> {
        if capacity > Self::MAX_SIZE {
            return Err("capacity exceeds max HeaderMap capacity");
        }
        Ok(HeaderMap {
            capacity,
            elements: Vec::with_capacity(capacity),
        })
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }
}

#[test]
fn test_header_map_with_capacity() {
    let map: HeaderMap<u32> = HeaderMap::with_capacity(10);

    assert!(map.is_empty());
    assert_eq!(10, map.capacity());
}

#[test]
#[should_panic(expected = "size overflows MAX_SIZE")]
fn test_header_map_with_capacity_exceeds_max() {
    let _map: HeaderMap<u32> = HeaderMap::with_capacity(101);
}

