// Answer 0

#[test]
fn test_is_hex() {
    assert_eq!(is_hex('0'), true); // Lower boundary for '0'
    assert_eq!(is_hex('1'), true);
    assert_eq!(is_hex('5'), true);
    assert_eq!(is_hex('9'), true); // Upper boundary for '9'
    assert_eq!(is_hex('a'), true);
    assert_eq!(is_hex('b'), true);
    assert_eq!(is_hex('f'), true);
    assert_eq!(is_hex('A'), true);
    assert_eq!(is_hex('B'), true);
    assert_eq!(is_hex('F'), true);
    assert_eq!(is_hex('g'), false); // Beyond 'f'
    assert_eq!(is_hex('-'), false); // Non-hex character
    assert_eq!(is_hex('Z'), false); // Beyond 'F'
}

