// Answer 0

#[test]
fn test_append_unchecked() {
    struct TestMap {
        indices: Vec<usize>,
        entries: Vec<(usize, usize)>,
    }

    impl TestMap {
        fn new() -> Self {
            Self {
                indices: Vec::new(),
                entries: Vec::new(),
            }
        }

        fn reserve(&mut self, additional: usize) {
            self.entries.reserve(additional);
        }

        fn len(&self) -> usize {
            self.entries.len()
        }
    }

    fn insert_bulk_no_grow(indices: &mut Vec<usize>, entries: &[(usize, usize)]) {
        for (i, _) in entries.iter().enumerate() {
            indices.push(i);
        }
    }

    let mut map1 = TestMap::new();
    let mut map2 = TestMap::new();

    map1.entries.push((1, 10));
    map1.entries.push((2, 20));

    map2.entries.push((3, 30));
    map2.entries.push((4, 40));

    // Simulating the append_unchecked function
    map1.append_unchecked(&mut map2);

    assert_eq!(map1.len(), 4);
    assert_eq!(map1.entries, vec![(1, 10), (2, 20), (3, 30), (4, 40)]);
    assert_eq!(map2.len(), 0);
}

