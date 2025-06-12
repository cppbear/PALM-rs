// Answer 0

#[test]
fn test_from_escape_table_carriage_return() {
    let escape = 13; // self::RR
    let byte = 100; // arbitrary value within the range
    from_escape_table(escape, byte);
}

#[test]
fn test_from_escape_table_carriage_return_with_zero_byte() {
    let escape = 13; // self::RR
    let byte = 0; // testing the lower edge of the range
    from_escape_table(escape, byte);
}

#[test]
fn test_from_escape_table_carriage_return_with_max_byte() {
    let escape = 13; // self::RR
    let byte = 255; // testing the upper edge of the range
    from_escape_table(escape, byte);
}

