// Answer 0

#[derive(Debug)]
enum Entry<V> {
    Vacant(V),
    Occupied(V),
}

#[test]
fn test_fmt_vacant() {
    let vacant_entry = Entry::Vacant("vacant_value");
    let result = format!("{:?}", vacant_entry);
    assert_eq!(result, "Entry(\"vacant_value\")");
}

#[test]
fn test_fmt_occupied() {
    let occupied_entry = Entry::Occupied("occupied_value");
    let result = format!("{:?}", occupied_entry);
    assert_eq!(result, "Entry(\"occupied_value\")");
}

