// Answer 0


#[derive(Debug)]
struct Bucket<K, V> {
    key: K,
    value: V,
}

struct Map<K, V> {
    entries: Vec<Bucket<K, V>>,
}

impl<K, V> Map<K, V> {
    pub fn new() -> Self {
        Map {
            entries: Vec::new(),
        }
    }

    pub fn push(&mut self, key: K, value: V) {
        self.entries.push(Bucket { key, value });
    }

    pub(crate) fn drain<R>(&mut self, range: R) -> std::vec::Drain<'_, Bucket<K, V>>
    where
        R: std::ops::RangeBounds<usize>,
    {
        let range = simplify_range(range, self.entries.len());
        self.erase_indices(range.start, range.end);
        self.entries.drain(range)
    }

    fn erase_indices(&mut self, start: usize, end: usize) {
        self.entries.drain(start..end);
    }
}

fn simplify_range<R>(range: R, len: usize) -> std::ops::Range<usize>
where
    R: std::ops::RangeBounds<usize>,
{
    let start = match range.start_bound() {
        std::ops::Bound::Included(&a) => a,
        std::ops::Bound::Excluded(&a) => a + 1,
        std::ops::Bound::Unbounded => 0,
    };

    let end = match range.end_bound() {
        std::ops::Bound::Included(&a) => a + 1,
        std::ops::Bound::Excluded(&a) => a,
        std::ops::Bound::Unbounded => len,
    };

    start..end
}

#[test]
fn test_drain_empty() {
    let mut map: Map<i32, i32> = Map::new();
    let drained: Vec<_> = map.drain(0..1).collect();
    assert!(drained.is_empty());
}

#[test]
fn test_drain_full() {
    let mut map = Map::new();
    map.push(1, 10);
    map.push(2, 20);
    map.push(3, 30);
    
    let drained: Vec<_> = map.drain(0..3).collect();
    assert_eq!(drained.len(), 3);
    assert_eq!(map.entries.len(), 0);
}

#[test]
fn test_drain_partial() {
    let mut map = Map::new();
    map.push(1, 10);
    map.push(2, 20);
    map.push(3, 30);
    
    let drained: Vec<_> = map.drain(1..3).collect();
    assert_eq!(drained.len(), 2);
    assert_eq!(map.entries.len(), 1);
    assert_eq!(map.entries[0].key, 1);
}

#[test]
fn test_drain_invalid_range() {
    let mut map = Map::new();
    map.push(1, 10);
    
    let drained: Vec<_> = map.drain(1..3).collect();
    assert_eq!(drained.len(), 0);
    assert_eq!(map.entries.len(), 1);
}


