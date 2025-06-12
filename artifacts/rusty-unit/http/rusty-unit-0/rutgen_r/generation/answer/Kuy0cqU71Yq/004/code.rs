// Answer 0

#[derive(Debug)]
struct Pos {
    entry: Option<(i32, i32)>, // tuple representing value and hash
}

impl Pos {
    fn resolve(&self) -> Option<(i32, i32)> {
        self.entry
    }
}

struct HeaderMap {
    indices: Vec<Pos>,
    mask: usize,
}

impl HeaderMap {
    fn new(mask: usize) -> Self {
        HeaderMap {
            indices: Vec::new(),
            mask,
        }
    }

    fn reinsert_entry_in_order(&mut self, pos: Pos) {
        if let Some((_, entry_hash)) = pos.resolve() {
            let mut probe = desired_pos(self.mask, entry_hash);
            probe_loop!(probe < self.indices.len(), {
                if self.indices[probe].resolve().is_none() {
                    // empty bucket, insert here
                    self.indices[probe] = pos;
                    return;
                }
            });
        }
    }
}

fn desired_pos(mask: usize, entry_hash: i32) -> usize {
    (entry_hash as usize) & mask
}

#[macro_export]
macro_rules! probe_loop {
    ($condition:expr, $body:block) => {
        while $condition {
            $body
        }
    };
}

#[test]
fn test_reinsert_entry_in_order_with_empty_indices() {
    let mut header_map = HeaderMap::new(7); // mask = 7
    let pos = Pos { entry: Some((1, 12)) }; // valid position

    header_map.reinsert_entry_in_order(pos);

    // The insert should succeed since indices are empty
    assert_eq!(header_map.indices.len(), 1);
}

#[test]
#[should_panic]
fn test_reinsert_entry_in_order_with_valid_entry_when_indices_are_empty() {
    let mut header_map = HeaderMap::new(0); // mask = 0
    let pos = Pos { entry: Some((2, 15)) }; // valid position

    header_map.reinsert_entry_in_order(pos);

    // This should panic because there are no buckets to insert into
}

