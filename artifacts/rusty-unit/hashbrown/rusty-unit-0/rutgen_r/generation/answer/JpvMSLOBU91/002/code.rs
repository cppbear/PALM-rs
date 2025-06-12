// Answer 0

fn test_entry_occupied_fmt() {
    use std::fmt;

    struct OccupiedEntry {
        key: String,
        value: i32,
    }

    enum Entry<'a> {
        Vacant(&'a str),
        Occupied(OccupiedEntry),
    }

    impl fmt::Debug for OccupiedEntry {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("OccupiedEntry")
                .field("key", &self.key)
                .field("value", &self.value)
                .finish()
        }
    }

    impl fmt::Debug for Entry<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match *self {
                Entry::Vacant(ref v) => f.debug_tuple("Entry").field(v).finish(),
                Entry::Occupied(ref o) => f.debug_tuple("Entry").field(o).finish(),
            }
        }
    }

    let occupied_entry = OccupiedEntry {
        key: String::from("example_key"),
        value: 42,
    };

    let entry = Entry::Occupied(occupied_entry);
    let output = format!("{:?}", entry);
    assert!(output.contains("OccupiedEntry"));
    assert!(output.contains("key: \"example_key\""));
    assert!(output.contains("value: 42"));
}

