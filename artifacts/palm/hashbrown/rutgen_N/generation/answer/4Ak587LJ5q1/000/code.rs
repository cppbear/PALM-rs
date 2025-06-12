// Answer 0

#[test]
fn test_fmt_vacant() {
    use std::fmt;

    struct VacantEntry;

    enum EntryRef<'a> {
        Vacant(&'a VacantEntry),
        Occupied(&'a str),
    }

    let vacant_entry = VacantEntry;
    let entry = EntryRef::Vacant(&vacant_entry);

    let mut output = String::new();
    {
        let writer = &mut fmt::Formatter::new(&mut output);
        entry.fmt(writer).unwrap();
    }

    assert!(output.contains("EntryRef"));
    assert!(output.contains("VacantEntry"));
}

#[test]
fn test_fmt_occupied() {
    use std::fmt;

    enum EntryRef<'a> {
        Vacant(&'a str),
        Occupied(&'a str),
    }

    let entry = EntryRef::Occupied("some_value");

    let mut output = String::new();
    {
        let writer = &mut fmt::Formatter::new(&mut output);
        entry.fmt(writer).unwrap();
    }

    assert!(output.contains("EntryRef"));
    assert!(output.contains("some_value"));
}

