// Answer 0

#[test]
fn test_compile_skip_table_empty() {
    let pattern: &[u8] = &[];
    let expected: Vec<usize> = vec![0; 256];
    assert_eq!(compile_skip_table(pattern), expected);
}

#[test]
fn test_compile_skip_table_single_character() {
    let pattern: &[u8] = &[b'a'];
    let expected: Vec<usize> = vec![1; 256];
    expected[b'a' as usize] = 0;
    assert_eq!(compile_skip_table(pattern), expected);
}

#[test]
fn test_compile_skip_table_multiple_characters() {
    let pattern: &[u8] = &[b'a', b'b', b'a'];
    let expected: Vec<usize> = vec![3; 256];
    expected[b'a' as usize] = 1; // Rightmost occurrence of 'a' is at index 2
    expected[b'b' as usize] = 0; // Rightmost occurrence of 'b' is at index 1
    assert_eq!(compile_skip_table(pattern), expected);
}

#[test]
fn test_compile_skip_table_full_range() {
    let pattern: &[u8] = &[
        b'a', b'b', b'c', b'd', b'e', b'f', b'g', b'h', b'i', b'j',
        b'k', b'l', b'm', b'n', b'o', b'p', b'q', b'r', b's', b't',
        b'u', b'v', b'w', b'x', b'y', b'z'
    ];
    let expected: Vec<usize> = vec![26; 256];
    for (i, &c) in pattern.iter().enumerate() {
        expected[c as usize] = pattern.len() - 1 - i;
    }
    assert_eq!(compile_skip_table(pattern), expected);
}

