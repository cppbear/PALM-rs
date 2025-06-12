// Answer 0

#[test]
fn test_compiler_new() {
    let compiler = Compiler::new();

    assert_eq!(compiler.insts.len(), 0);
    assert_eq!(compiler.num_exprs, 0);
    assert_eq!(compiler.size_limit, 10 * (1 << 20));
    assert_eq!(compiler.capture_name_idx.len(), 0);
    
    let program = &compiler.compiled;
    assert_eq!(program.insts.len(), 0);
    assert_eq!(program.matches.len(), 0);
    assert!(program.captures.is_empty());
    assert_eq!(Arc::strong_count(&program.capture_name_idx), 1);
    assert_eq!(program.start, 0);
    assert_eq!(program.byte_classes.len(), 256);
    assert!(program.only_utf8);
    assert!(!program.is_bytes);
    assert!(!program.is_dfa);
    assert!(!program.is_reverse);
    assert!(!program.is_anchored_start);
    assert!(!program.is_anchored_end);
    assert!(!program.has_unicode_word_boundary);
    
    let suffix_cache = &compiler.suffix_cache;
    assert_eq!(suffix_cache.table.len(), 1000);
    assert_eq!(suffix_cache.version, 0);
    
    assert_eq!(compiler.utf8_seqs, Some(Utf8Sequences::new('\x00', '\x00')));
    
    let byte_class_set = &compiler.byte_classes;
    let byte_classes_array = byte_class_set.0;
    for &entry in &byte_classes_array {
        assert!(!entry);
    }
}

