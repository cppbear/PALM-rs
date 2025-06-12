// Answer 0

#[derive(Debug)]
struct Vacant;
#[derive(Debug)]
struct Occupied;

enum RawEntryMut {
    Vacant(Vacant),
    Occupied(Occupied),
}

impl std::fmt::Debug for RawEntryMut {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut tuple = f.debug_tuple("RawEntryMut");
        match self {
            Self::Vacant(v) => tuple.field(v),
            Self::Occupied(o) => tuple.field(o),
        };
        tuple.finish()
    }
}

#[test]
fn test_raw_entry_mut_fmt_vacant() {
    let entry = RawEntryMut::Vacant(Vacant);
    let mut output = String::new();
    let result = std::fmt::write(&mut output, format_args!("{:?}", entry));
    assert!(result.is_ok());
    assert_eq!(output, "RawEntryMut(\n    Vacant,\n)");
}

#[test]
fn test_raw_entry_mut_fmt_occupied() {
    let entry = RawEntryMut::Occupied(Occupied);
    let mut output = String::new();
    let result = std::fmt::write(&mut output, format_args!("{:?}", entry));
    assert!(result.is_ok());
    assert_eq!(output, "RawEntryMut(\n    Occupied,\n)");
}

