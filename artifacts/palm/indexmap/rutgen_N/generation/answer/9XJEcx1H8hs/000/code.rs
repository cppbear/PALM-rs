// Answer 0

#[test]
fn test_split_first_mut_non_empty() {
    struct TestStruct {
        entries: Vec<(i32, String)>,
    }

    impl TestStruct {
        fn from_mut_slice(slice: &mut [(i32, String)]) -> Self {
            TestStruct {
                entries: slice.to_vec(),
            }
        }
        
        fn split_first_mut(&mut self) -> Option<((&i32, &mut String), &mut Self)> {
            if let [first, rest @ ..] = &mut self.entries[..] {
                Some((first, Self::from_mut_slice(rest)))
            } else {
                None
            }
        }
    }

    let mut test_struct = TestStruct {
        entries: vec![(1, String::from("one")), (2, String::from("two"))],
    };
    let result = test_struct.split_first_mut();
    assert!(result.is_some());

    let ((key, value), rest) = result.unwrap();
    assert_eq!(*key, 1);
    *value = String::from("updated");
    assert_eq!(test_struct.entries[0].1, "updated");
    assert_eq!(rest.entries.len(), 1);
}

#[test]
fn test_split_first_mut_empty() {
    struct TestStruct {
        entries: Vec<(i32, String)>,
    }

    impl TestStruct {
        fn from_mut_slice(slice: &mut [(i32, String)]) -> Self {
            TestStruct {
                entries: slice.to_vec(),
            }
        }
        
        fn split_first_mut(&mut self) -> Option<((&i32, &mut String), &mut Self)> {
            if let [first, rest @ ..] = &mut self.entries[..] {
                Some((first, Self::from_mut_slice(rest)))
            } else {
                None
            }
        }
    }

    let mut test_struct = TestStruct { entries: vec![] };
    let result = test_struct.split_first_mut();
    assert!(result.is_none());
}

