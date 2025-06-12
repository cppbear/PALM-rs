// Answer 0

#[derive(Debug)]
enum Entry<T> {
    Vacant(T),
    Occupied(T),
}

impl<T: std::fmt::Debug> std::fmt::Debug for Entry<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Entry::Vacant(ref v) => f.debug_tuple("Entry").field(v).finish(),
            Entry::Occupied(ref o) => f.debug_tuple("Entry").field(o).finish(),
        }
    }
}

#[test]
fn test_entry_vacant_fmt() {
    let entry = Entry::Vacant(42);
    let result = format!("{:?}", entry);
    assert_eq!(result, "Entry(42)");
}

#[test]
fn test_entry_occupied_fmt() {
    let entry = Entry::Occupied(99);
    let result = format!("{:?}", entry);
    assert_eq!(result, "Entry(99)");
}

