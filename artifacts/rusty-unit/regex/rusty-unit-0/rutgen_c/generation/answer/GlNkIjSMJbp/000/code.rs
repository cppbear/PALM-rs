// Answer 0

#[test]
fn test_is_eof() {
    let eof_byte = Byte::eof();
    let non_eof_byte = Byte::byte(100);

    assert!(eof_byte.is_eof());
    assert!(!non_eof_byte.is_eof());
}

#[test]
fn test_is_eof_boundary_condition() {
    let boundary_byte = Byte::byte(255);
    
    assert!(!boundary_byte.is_eof());
}

