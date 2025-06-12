// Answer 0

#[test]
fn test_from_escape_table_reverse_solidus_with_zero_byte() {
    let escape = 92; // BS
    let byte = 0;
    from_escape_table(escape, byte);
}

#[test]
fn test_from_escape_table_reverse_solidus_with_max_byte() {
    let escape = 92; // BS
    let byte = 255;
    from_escape_table(escape, byte);
}

#[test]
fn test_from_escape_table_reverse_solidus_with_mid_byte() {
    let escape = 92; // BS
    let byte = 128;
    from_escape_table(escape, byte);
}

#[test]
fn test_from_escape_table_reverse_solidus_with_random_byte() {
    let escape = 92; // BS
    let byte = 127; // another byte within range
    from_escape_table(escape, byte);
}

