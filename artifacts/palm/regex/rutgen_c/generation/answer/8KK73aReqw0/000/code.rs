// Answer 0

#[test]
fn test_program_new_empty() {
    let program = Program::new();
    
    // Verify the default state of the Program
    assert_eq!(program.insts.len(), 0);
    assert_eq!(program.matches.len(), 0);
    assert_eq!(program.captures.len(), 0);
    assert!(Arc::strong_count(&program.capture_name_idx) == 1);
    assert_eq!(program.start, 0);
    assert_eq!(program.byte_classes.len(), 256);
    assert!(program.only_utf8);
    assert!(!program.is_bytes);
    assert!(!program.is_dfa);
    assert!(!program.is_reverse);
    assert!(!program.is_anchored_start);
    assert!(!program.is_anchored_end);
    assert!(!program.has_unicode_word_boundary);
    assert_eq!(program.dfa_size_limit, 2 * (1 << 20));
}

