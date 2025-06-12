// Answer 0

#[test]
fn test_fmt_vacant_entry() {
    use std::fmt;

    struct Vacant;
    
    enum Entry<'a> {
        Vacant(Vacant),
        Occupied(&'a str),
    }

    let vacant_entry = Entry::Vacant(Vacant);
    let mut output = String::new();
    let result = fmt::write(&mut output, |f| vacant_entry.fmt(f));
    
    assert!(result.is_ok());
    assert_eq!(output, "Entry(Vacant)");
}

#[test]
fn test_fmt_occupied_entry() {
    use std::fmt;

    struct Occupied;
    
    enum Entry<'a> {
        Vacant(Vacant),
        Occupied(&'a str),
    }

    let occupied_entry = Entry::Occupied("value");
    let mut output = String::new();
    let result = fmt::write(&mut output, |f| occupied_entry.fmt(f));
    
    assert!(result.is_ok());
    assert_eq!(output, "Entry(Occupied(\"value\"))");
}

