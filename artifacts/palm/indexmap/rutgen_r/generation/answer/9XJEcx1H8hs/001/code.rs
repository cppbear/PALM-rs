// Answer 0

#[test]
fn test_split_first_mut_non_empty() {
    struct DummyEntry<'a> {
        key: &'a str,
        value: i32,
    }

    struct DummyMap {
        entries: Vec<DummyEntry<'static>>,
    }

    impl DummyMap {
        fn from_mut_slice(entries: &mut [DummyEntry<'static>]) -> &mut Self {
            unsafe { &mut *(entries as *mut _ as *mut DummyMap) }
        }

        fn split_first_mut(&mut self) -> Option<((&str, &mut i32), &mut Self)> {
            if let [first, rest @ ..] = &mut self.entries[..] {
                Some((first.key, first.value.to_mut(), Self::from_mut_slice(rest)))
            } else {
                None
            }
        }
    }

    let mut map = DummyMap {
        entries: vec![
            DummyEntry { key: "a", value: 1 },
            DummyEntry { key: "b", value: 2 },
        ],
    };

    let result = map.split_first_mut();
    assert!(result.is_some());

    if let Some(((key, value), _rest)) = result {
        assert_eq!(key, "a");
        *value += 1;
        assert_eq!(map.entries[0].value, 2);
    }
}

#[test]
fn test_split_first_mut_empty() {
    struct DummyEntry<'a> {
        key: &'a str,
        value: i32,
    }

    struct DummyMap {
        entries: Vec<DummyEntry<'static>>,
    }

    impl DummyMap {
        fn from_mut_slice(entries: &mut [DummyEntry<'static>]) -> &mut Self {
            unsafe { &mut *(entries as *mut _ as *mut DummyMap) }
        }

        fn split_first_mut(&mut self) -> Option<((&str, &mut i32), &mut Self)> {
            if let [first, rest @ ..] = &mut self.entries[..] {
                Some((first.key, first.value.to_mut(), Self::from_mut_slice(rest)))
            } else {
                None
            }
        }
    }

    let mut map = DummyMap { entries: vec![] };

    let result = map.split_first_mut();
    assert!(result.is_none());
}

