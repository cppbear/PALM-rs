// Answer 0

#[test]
fn test_compile_md2_shift_multiple_occurrences() {
    let pattern: Vec<u8> = b"abcabc".to_vec();
    let result = BoyerMooreSearch::compile_md2_shift(&pattern);
    assert_eq!(result, 3);
}

#[test]
fn test_compile_md2_shift_single_occurrence() {
    let pattern: Vec<u8> = b"abc".to_vec();
    let result = BoyerMooreSearch::compile_md2_shift(&pattern);
    assert_eq!(result, 2);
}

#[test]
fn test_compile_md2_shift_only_one_char() {
    let pattern: Vec<u8> = b"a".to_vec();
    let result = BoyerMooreSearch::compile_md2_shift(&pattern);
    assert_eq!(result, 0xDEADBEAF);
}

#[test]
fn test_compile_md2_shift_no_occurrence() {
    let pattern: Vec<u8> = b"abcdef".to_vec();
    let result = BoyerMooreSearch::compile_md2_shift(&pattern);
    assert_eq!(result, 5);
}

