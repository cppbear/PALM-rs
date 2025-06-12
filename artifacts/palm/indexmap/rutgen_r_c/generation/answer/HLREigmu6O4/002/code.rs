// Answer 0

#[test]
fn test_replace_full_none() {
    use std::collections::hash_map::RandomState;
    
    struct TestIndexSet {
        map: IndexMap<i32, (), RandomState>,
    }

    impl TestIndexSet {
        fn new() -> Self {
            let map = IndexMap {
                core: IndexMapCore {
                    indices: Indices::new(),
                    entries: Entries::new(),
                },
                hash_builder: RandomState::new(),
            };
            TestIndexSet { map }
        }

        fn replace_full(&mut self, value: i32) -> (usize, Option<i32>) {
            let hash = self.map.hash(&value);
            match self.map.core.replace_full(hash, value, ()) {
                (i, Some((replaced, ()))) => (i, Some(replaced)),
                (i, None) => (i, None),
            }
        }
    }

    let mut set = TestIndexSet::new();
    let (index, replaced_value) = set.replace_full(42);
    assert_eq!(index, 0);
    assert_eq!(replaced_value, None);
}

