// Answer 0

fn test_entry_fmt_occupied() {
    use std::fmt;

    struct OccupiedEntry {
        data: i32,
    }

    enum Entry<'a> {
        Vacant(&'a str),
        Occupied(OccupiedEntry),
    }

    impl fmt::Debug for Entry<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match *self {
                Entry::Vacant(ref v) => f.debug_tuple("Entry").field(v).finish(),
                Entry::Occupied(ref o) => f.debug_tuple("Entry").field(o).finish(),
            }
        }
    }

    let entry = Entry::Occupied(OccupiedEntry { data: 42 });
    let mut buffer = String::new();
    let result = write!(&mut buffer, "{:?}", entry);
    assert!(result.is_ok());
    assert_eq!(buffer, "Entry(OccupiedEntry { data: 42 })");

    // Test with additional data
    let entry_with_different_data = Entry::Occupied(OccupiedEntry { data: 100 });
    buffer.clear();
    let result = write!(&mut buffer, "{:?}", entry_with_different_data);
    assert!(result.is_ok());
    assert_eq!(buffer, "Entry(OccupiedEntry { data: 100 })");
}

