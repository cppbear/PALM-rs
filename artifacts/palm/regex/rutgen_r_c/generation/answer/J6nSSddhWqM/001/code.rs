// Answer 0

#[test]
fn test_len_utf8_valid_char() {
    let valid_char_a = Char(97); // 'a'
    assert_eq!(valid_char_a.len_utf8(), 1);
    
    let valid_char_aa = Char(2048); // 'ŀ'
    assert_eq!(valid_char_aa.len_utf8(), 2);

    let valid_char_aaa = Char(65533); // '�' (replacement character)
    assert_eq!(valid_char_aaa.len_utf8(), 3);
}

#[test]
fn test_len_utf8_none() {
    let none_char = Char(0); // invalid Unicode character
    assert_eq!(none_char.len_utf8(), 0);
}

#[test]
fn test_len_utf8_boundary_values() {
    let boundary_char_min = Char(1); // non-printable character
    assert_eq!(boundary_char_min.len_utf8(), 1);

    let boundary_char_max = Char(u32::MAX); // out of valid Unicode range
    assert_eq!(boundary_char_max.len_utf8(), 0);
}

