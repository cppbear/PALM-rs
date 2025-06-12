// Answer 0


#[derive(Default)]
struct Entries {
    entries: Vec<i32>,
}

struct IndexMap {
    entries: Entries,
}

impl IndexMap {
    fn len(&self) -> usize {
        self.entries.entries.len()
    }

    fn erase_indices(&mut self, start: usize, end: usize) {
        self.entries.entries.drain(start..end);
    }

    pub(crate) fn truncate(&mut self, len: usize) {
        if len < self.len() {
            self.erase_indices(len, self.entries.entries.len());
            self.entries.entries.truncate(len);
        }
    }
}

#[test]
fn test_truncate_valid_input() {
    let mut map = IndexMap {
        entries: Entries {
            entries: vec![1, 2, 3, 4, 5], // initial length is 5
        },
    };
    map.truncate(3); // truncate to 3, should work without panic
    assert_eq!(map.len(), 3);
    assert_eq!(map.entries.entries, vec![1, 2, 3]);
}

#[test]
fn test_truncate_edge_case() {
    let mut map = IndexMap {
        entries: Entries {
            entries: vec![10, 20, 30], // initial length is 3
        },
    };
    map.truncate(2); // truncate to 2, should work without panic
    assert_eq!(map.len(), 2);
    assert_eq!(map.entries.entries, vec![10, 20]);
}


