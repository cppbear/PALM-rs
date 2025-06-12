// Answer 0

#[test]
fn test_as_entries() {
    struct TestCore {
        entries: Vec<(i32, i32)>,
    }

    impl TestCore {
        fn as_entries(&self) -> &[(i32, i32)] {
            &self.entries
        }
    }

    struct TestMap {
        core: TestCore,
    }

    impl TestMap {
        fn new(entries: Vec<(i32, i32)>) -> Self {
            Self {
                core: TestCore { entries },
            }
        }

        fn as_entries(&self) -> &[(i32, i32)] {
            self.core.as_entries()
        }
    }

    let test_map = TestMap::new(vec![(1, 2), (3, 4), (5, 6)]);
    let entries = test_map.as_entries();

    assert_eq!(entries.len(), 3);
    assert_eq!(entries[0], (1, 2));
    assert_eq!(entries[1], (3, 4));
    assert_eq!(entries[2], (5, 6));
}

