// Answer 0

#[test]
fn test_compile_skip_table_empty_pattern() {
    let pattern: Vec<u8> = Vec::new();
    let skip_table = BoyerMooreSearch::compile_skip_table(&pattern);
    assert_eq!(skip_table, vec![0; 256]);
}

#[test]
fn test_compile_skip_table_single_character() {
    let pattern = vec![b'a'];
    let skip_table = BoyerMooreSearch::compile_skip_table(&pattern);
    assert_eq!(skip_table[b'a' as usize], 0);
    assert_eq!(skip_table[b'b' as usize], 1);
    assert_eq!(skip_table[b'c' as usize], 1);
}

#[test]
fn test_compile_skip_table_multiple_characters() {
    let pattern = vec![b'a', b'b', b'a'];
    let skip_table = BoyerMooreSearch::compile_skip_table(&pattern);
    assert_eq!(skip_table[b'a' as usize], 1);
    assert_eq!(skip_table[b'b' as usize], 0);
    assert_eq!(skip_table[b'c' as usize], 2);
}

#[test]
fn test_compile_skip_table_all_unique_characters() {
    let pattern = b"abcdef".to_vec();
    let skip_table = BoyerMooreSearch::compile_skip_table(&pattern);
    assert_eq!(skip_table[b'a' as usize], 5);
    assert_eq!(skip_table[b'b' as usize], 4);
    assert_eq!(skip_table[b'c' as usize], 3);
    assert_eq!(skip_table[b'd' as usize], 2);
    assert_eq!(skip_table[b'e' as usize], 1);
    assert_eq!(skip_table[b'f' as usize], 0);
}

#[test]
fn test_compile_skip_table_non_ascii_characters() {
    let pattern = vec![240, 159, 152, 138, 240, 159, 152, 139]; // represents some emojis
    let skip_table = BoyerMooreSearch::compile_skip_table(&pattern);
    assert_eq!(skip_table[240], 1); // checking first emoji
    assert_eq!(skip_table[159], 0); // checking second emoji
}

