// Answer 0

#[derive(Debug)]
struct OccupiedEntry {
    key: String,
    value: i32,
}

enum RawEntryMut {
    Vacant(String),
    Occupied(OccupiedEntry),
}

impl core::fmt::Debug for RawEntryMut {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut tuple = f.debug_tuple("RawEntryMut");
        match self {
            Self::Vacant(v) => tuple.field(v),
            Self::Occupied(o) => tuple.field(o),
        };
        tuple.finish()
    }
}

#[test]
fn test_fmt_occupied_entry() {
    let entry = RawEntryMut::Occupied(OccupiedEntry {
        key: String::from("key1"),
        value: 42,
    });

    let result = format!("{:?}", entry);
    assert!(result.contains("OccupiedEntry { key: \"key1\", value: 42 }"));
}

#[test]
fn test_fmt_different_occupied_entry() {
    let entry = RawEntryMut::Occupied(OccupiedEntry {
        key: String::from("key2"),
        value: 100,
    });

    let result = format!("{:?}", entry);
    assert!(result.contains("OccupiedEntry { key: \"key2\", value: 100 }"));
}

