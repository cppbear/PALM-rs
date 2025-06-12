// Answer 0

#[test]
fn test_compile_md2_shift_single_character_pattern() {
    let pattern = vec![b'a']; // pattern.len() == 1, should return 0xDEADBEAF
    assert_eq!(compile_md2_shift(&pattern), 0xDEADBEAF);
}

#[test]
fn test_compile_md2_shift_no_occurrence_of_skip_char() {
    let pattern = vec![b'a', b'b', b'c', b'd']; // skip char 'd' does not re-occur
    assert_eq!(compile_md2_shift(&pattern), 3); // whole window length
}

#[test]
fn test_compile_md2_shift_with_reoccurrence_of_skip_char() {
    let pattern = vec![b'a', b'b', b'c', b'd', b'b', b'd']; // skip char 'd' re-occurs
    assert_eq!(compile_md2_shift(&pattern), 2); // shift to occurrence of 'd'
}

#[test]
fn test_compile_md2_shift_multiple_occurrences() {
    let pattern = vec![b'a', b'd', b'b', b'd']; // skip char 'd' re-occurs
    assert_eq!(compile_md2_shift(&pattern), 1); // shift to occurrence of 'd'
}

#[test]
fn test_compile_md2_shift_edge_case() {
    let pattern = vec![b'a', b'a']; // skip char 'a' re-occurs
    assert_eq!(compile_md2_shift(&pattern), 1); // shift to occurrence of 'a'
}

