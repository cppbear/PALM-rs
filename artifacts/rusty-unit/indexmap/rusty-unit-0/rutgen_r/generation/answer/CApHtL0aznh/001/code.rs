// Answer 0

#[test]
fn test_key_mut_vacant_entry() {
    use indexmap::IndexMap;
    use indexmap::indexmap;

    struct MyStruct {
        key: String,
    }

    struct VacantEntry {
        key: String,
    }

    struct Entry<'a> {
        occupied: Option<&'a MyStruct>,
        vacant: Option<&'a mut VacantEntry>,
    }

    impl<'a> Entry<'a> {
        fn key_mut(&mut self) -> &mut String {
            match self {
                Entry { occupied: Some(e), .. } => {
                    // This should never happen in this test as we're testing Vacant scenario
                    panic!("Expected Vacant entry, found Occupied");
                }
                Entry { vacant: Some(e), .. } => &mut e.key,
            }
        }
    }
    
    let mut vacant_entry = VacantEntry {
        key: String::from("initial_key"),
    };
    let mut entry = Entry {
        occupied: None,
        vacant: Some(&mut vacant_entry),
    };

    let key = entry.key_mut();
    *key = String::from("updated_key");

    assert_eq!(vacant_entry.key, "updated_key");
}

