// Answer 0

#[test]
fn test_program_new_empty() {
    let program = Program::new();
}

#[test]
fn test_program_new_invariants() {
    let program = Program::new();
    let empty_hashmap: Arc<HashMap<String, usize>> = Arc::new(HashMap::new());
    
    assert_eq!(program.insts.len(), 0);
    assert_eq!(program.matches.len(), 0);
    assert_eq!(program.captures.len(), 0);
    assert_eq!(Arc::ptr_eq(&program.capture_name_idx, &empty_hashmap), true);
    assert_eq!(program.start, 0);
    assert_eq!(program.byte_classes.len(), 256);
    assert_eq!(program.only_utf8, true);
    assert_eq!(program.is_bytes, false);
    assert_eq!(program.is_dfa, false);
    assert_eq!(program.is_reverse, false);
    assert_eq!(program.is_anchored_start, false);
    assert_eq!(program.is_anchored_end, false);
    assert_eq!(program.has_unicode_word_boundary, false);
    assert!(program.prefixes.is_empty());
    assert_eq!(program.dfa_size_limit, 2097152);
}

