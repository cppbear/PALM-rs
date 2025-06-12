// Answer 0

#[test]
fn test_split_last_mut_non_empty() {
    struct TestMap {
        entries: Vec<(i32, String)>,
    }

    impl TestMap {
        pub fn from_mut_slice(slice: &mut [(i32, String)]) -> Self {
            TestMap {
                entries: slice.to_vec(),
            }
        }

        pub fn split_last_mut(&mut self) -> Option<((&i32, &mut String), &mut Self)> {
            if let [rest @ .., last] = &mut self.entries {
                Some((last.ref_mut(), Self::from_mut_slice(rest)))
            } else {
                None
            }
        }
    }

    impl TestMap {
        fn ref_mut(&mut self) -> (&i32, &mut String) {
            let (key, value) = &mut self.entries.last_mut().unwrap();
            (key, value)
        }
    }

    let mut map = TestMap {
        entries: vec![(1, String::from("one")), (2, String::from("two"))],
    };

    if let Some((last, rest)) = map.split_last_mut() {
        assert_eq!(*last.0, 2);
        assert_eq!(last.1, &mut String::from("two"));
        assert_eq!(rest.entries.len(), 1);
        assert_eq!(rest.entries[0].0, 1);
    } else {
        panic!("Expected Some result, got None");
    }
}

#[test]
fn test_split_last_mut_empty() {
    struct TestMap {
        entries: Vec<(i32, String)>,
    }

    impl TestMap {
        pub fn from_mut_slice(slice: &mut [(i32, String)]) -> Self {
            TestMap {
                entries: slice.to_vec(),
            }
        }

        pub fn split_last_mut(&mut self) -> Option<((&i32, &mut String), &mut Self)> {
            if let [rest @ .., last] = &mut self.entries {
                Some((last.ref_mut(), Self::from_mut_slice(rest)))
            } else {
                None
            }
        }
    }

    let mut map = TestMap {
        entries: Vec::new(),
    };

    let result = map.split_last_mut();
    assert!(result.is_none());
}

