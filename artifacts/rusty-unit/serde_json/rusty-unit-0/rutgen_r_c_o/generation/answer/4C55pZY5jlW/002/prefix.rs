// Answer 0

#[test]
fn test_from_escape_table_with_valid_ascii_control_byte_1() {
    let escape = UU; 
    let byte = 0x01; 
    from_escape_table(escape, byte);
}

#[test]
fn test_from_escape_table_with_valid_ascii_control_byte_2() {
    let escape = UU; 
    let byte = 0x7F; 
    from_escape_table(escape, byte);
}

#[test]
fn test_from_escape_table_with_valid_ascii_control_byte_3() {
    let escape = UU; 
    let byte = 0xFF; 
    from_escape_table(escape, byte);
}

#[test]
fn test_from_escape_table_with_zero_byte() {
    let escape = UU; 
    let byte = 0x00; 
    from_escape_table(escape, byte);
}

