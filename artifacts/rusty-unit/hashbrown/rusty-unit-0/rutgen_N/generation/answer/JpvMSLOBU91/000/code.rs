// Answer 0

#[test]
fn test_entry_fmt_vacant() {
    use std::fmt;
    use hashbrown::map::Entry;

    struct Vacant;

    impl fmt::Debug for Vacant {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "Vacant Entry")
        }
    }

    let vacant_entry = Entry::Vacant(Vacant);
    let mut buffer = String::new();
    let result = write!(&mut buffer, "{:?}", vacant_entry);

    assert!(result.is_ok());
    assert_eq!(buffer, "Entry(Vacant Entry)");
}

#[test]
fn test_entry_fmt_occupied() {
    use std::fmt;
    use hashbrown::map::Entry;

    struct Occupied;

    impl fmt::Debug for Occupied {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "Occupied Entry")
        }
    }

    let occupied_entry = Entry::Occupied(Occupied);
    let mut buffer = String::new();
    let result = write!(&mut buffer, "{:?}", occupied_entry);

    assert!(result.is_ok());
    assert_eq!(buffer, "Entry(Occupied Entry)");
}

