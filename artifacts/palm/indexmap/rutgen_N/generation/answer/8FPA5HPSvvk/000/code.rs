// Answer 0

#[test]
fn test_last_mut_some() {
    struct TestMap {
        entries: Vec<(i32, String)>,
    }

    impl TestMap {
        fn as_entries_mut(&mut self) -> &mut Vec<(i32, String)> {
            &mut self.entries
        }

        fn last_mut(&mut self) -> Option<(&i32, &mut String)> {
            self.as_entries_mut().last_mut().map(|(k, v)| (k, v))
        }
    }

    let mut map = TestMap { entries: vec![(1, "one".to_string()), (2, "two".to_string())] };
    if let Some((key, value)) = map.last_mut() {
        assert_eq!(*key, 2);
        *value = "updated".to_string();
    }
    assert_eq!(map.entries.last().unwrap().1, "updated");
}

#[test]
fn test_last_mut_none() {
    struct TestMap {
        entries: Vec<(i32, String)>,
    }

    impl TestMap {
        fn as_entries_mut(&mut self) -> &mut Vec<(i32, String)> {
            &mut self.entries
        }

        fn last_mut(&mut self) -> Option<(&i32, &mut String)> {
            self.as_entries_mut().last_mut().map(|(k, v)| (k, v))
        }
    }

    let mut map = TestMap { entries: Vec::new() };
    assert_eq!(map.last_mut(), None);
}

