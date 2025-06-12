// Answer 0

#[test]
fn test_erase_index_success() {
    struct Indices {
        indices: Vec<usize>,
    }

    impl Indices {
        fn new() -> Self {
            Indices { indices: vec![0, 1, 2, 3, 4] }
        }

        fn find_entry(&mut self, hash: usize, predicate: impl Fn(usize) -> bool) -> Result<Entry, ()> {
            if hash < self.indices.len() && predicate(self.indices[hash]) {
                Ok(Entry { index: hash, indices: self })
            } else {
                Err(())
            }
        }
    }

    struct Entry<'a> {
        index: usize,
        indices: &'a mut Indices,
    }

    impl<'a> Entry<'a> {
        fn remove(self) {
            self.indices.indices.remove(self.index);
        }
    }

    struct HashValue(usize);

    impl HashValue {
        fn get(&self) -> usize {
            self.0
        }
    }

    let mut table = Indices::new();
    let hash = HashValue(2);
    let index = 2;

    erase_index(&mut table, hash, index);
    assert_eq!(table.indices, vec![0, 1, 3, 4]);
}

#[test]
#[should_panic(expected = "index not found")]
fn test_erase_index_not_found() {
    struct Indices {
        indices: Vec<usize>,
    }

    impl Indices {
        fn new() -> Self {
            Indices { indices: vec![0, 1, 2, 3, 4] }
        }

        fn find_entry(&mut self, hash: usize, predicate: impl Fn(usize) -> bool) -> Result<Entry, ()> {
            if hash < self.indices.len() && predicate(self.indices[hash]) {
                Ok(Entry { index: hash, indices: self })
            } else {
                Err(())
            }
        }
    }

    struct Entry<'a> {
        index: usize,
        indices: &'a mut Indices,
    }

    impl<'a> Entry<'a> {
        fn remove(self) {
            self.indices.indices.remove(self.index);
        }
    }

    struct HashValue(usize);

    impl HashValue {
        fn get(&self) -> usize {
            self.0
        }
    }

    let mut table = Indices::new();
    let hash = HashValue(5);
    let index = 5;

    erase_index(&mut table, hash, index);
}

