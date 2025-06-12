// Answer 0

#[test]
fn test_fmt_occupied_entry() {
    use std::fmt;

    struct OccupiedEntry {
        value: i32,
    }

    enum Entry {
        Vacant(String),
        Occupied(OccupiedEntry),
    }

    impl fmt::Debug for Entry {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match *self {
                Entry::Vacant(ref v) => f.debug_tuple("Entry").field(v).finish(),
                Entry::Occupied(ref o) => f.debug_tuple("Entry").field(o).finish(),
            }
        }
    }

    impl fmt::Debug for OccupiedEntry {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("OccupiedEntry")
                .field("value", &self.value)
                .finish()
        }
    }

    let occupied_entry = Entry::Occupied(OccupiedEntry { value: 42 });
    let mut output = String::new();
    let result = write!(&mut output, "{:?}", occupied_entry);
    
    assert!(result.is_ok());
    assert!(output.contains("Entry"));
    assert!(output.contains("OccupiedEntry"));
    assert!(output.contains("value: 42"));
}

