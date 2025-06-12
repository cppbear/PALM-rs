// Answer 0

#[test]
fn test_last_on_empty_index_map() {
    struct DummyIndexMap {
        entries: Vec<(i32, i32)>,
    }

    impl DummyIndexMap {
        pub fn last(&self) -> Option<(&i32, &i32)> {
            self.entries.last()
        }
    }

    let map = DummyIndexMap { entries: vec![] };
    assert_eq!(map.last(), None);
}

#[test]
fn test_last_on_non_empty_index_map() {
    struct DummyIndexMap {
        entries: Vec<(i32, i32)>,
    }

    impl DummyIndexMap {
        pub fn last(&self) -> Option<(&i32, &i32)> {
            self.entries.last()
        }
    }

    let map = DummyIndexMap {
        entries: vec![(1, 10), (2, 20)],
    };
    assert_eq!(map.last(), Some((&2, &20)));
}

