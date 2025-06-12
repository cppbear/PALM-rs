// Answer 0

#[test]
fn test_compile_skip_table_empty() {
    let pattern: &[u8] = b"";
    let result = compile_skip_table(pattern);
    assert_eq!(result, vec![0; 256]);
}

#[test]
fn test_compile_skip_table_single_char() {
    let pattern: &[u8] = b"a";
    let result = compile_skip_table(pattern);
    let expected = vec![1; 256];
    expected[b'a' as usize] = 0;
    assert_eq!(result, expected);
}

#[test]
fn test_compile_skip_table_multiple_chars() {
    let pattern: &[u8] = b"abcabc";
    let result = compile_skip_table(pattern);
    let mut expected = vec![6; 256];
    expected[b'a' as usize] = 5;
    expected[b'b' as usize] = 4;
    expected[b'c' as usize] = 3;
    assert_eq!(result, expected);
}

#[test]
fn test_compile_skip_table_non_ascii() {
    let pattern: &[u8] = b"\x80\x81\x82\x80";
    let result = compile_skip_table(pattern);
    let mut expected = vec![4; 256];
    expected[0x80] = 3;
    expected[0x81] = 2;
    expected[0x82] = 1;
    assert_eq!(result, expected);
}

