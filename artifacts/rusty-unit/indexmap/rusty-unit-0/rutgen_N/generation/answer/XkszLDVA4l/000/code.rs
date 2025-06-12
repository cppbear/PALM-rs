// Answer 0

#[derive(Debug, Default)]
struct Indices {
    data: Vec<usize>,
}

impl Indices {
    fn with_capacity(capacity: usize) -> Self {
        Self {
            data: Vec::with_capacity(capacity),
        }
    }
}

struct Map {
    indices: Indices,
    entries: Vec<i32>,
}

impl Map {
    pub(crate) fn split_off(&mut self, at: usize) -> Self {
        let len = self.entries.len();
        assert!(
            at <= len,
            "index out of bounds: the len is {len} but the index is {at}. Expected index <= len"
        );

        self.erase_indices(at, self.entries.len());
        let entries = self.entries.split_off(at);

        let mut indices = Indices::with_capacity(entries.len());
        insert_bulk_no_grow(&mut indices, &entries);
        Self { indices, entries }
    }

    fn erase_indices(&mut self, start: usize, end: usize) {
        self.indices.data.drain(start..end);
    }
}

fn insert_bulk_no_grow(indices: &mut Indices, entries: &[i32]) {
    indices.data.extend(0..entries.len());
}

#[test]
fn test_split_off_with_valid_index() {
    let mut map = Map {
        indices: Indices::default(),
        entries: vec![1, 2, 3, 4, 5],
    };
    let split = map.split_off(3);
    assert_eq!(map.entries, vec![1, 2, 3]);
    assert_eq!(split.entries, vec![4, 5]);
    assert_eq!(split.indices.data.len(), 2);
}

#[test]
#[should_panic(expected = "index out of bounds: the len is 5 but the index is 6. Expected index <= len")]
fn test_split_off_with_out_of_bounds_index() {
    let mut map = Map {
        indices: Indices::default(),
        entries: vec![1, 2, 3, 4, 5],
    };
    map.split_off(6);
}

#[test]
fn test_split_off_with_zero_index() {
    let mut map = Map {
        indices: Indices::default(),
        entries: vec![1, 2, 3, 4, 5],
    };
    let split = map.split_off(0);
    assert_eq!(map.entries, vec![]);
    assert_eq!(split.entries, vec![1, 2, 3, 4, 5]);
    assert_eq!(split.indices.data.len(), 5);
}

