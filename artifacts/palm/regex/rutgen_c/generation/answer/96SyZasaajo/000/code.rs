// Answer 0

#[test]
fn test_compiler_new() {
    let compiler = Compiler::new();
    assert_eq!(compiler.insts.len(), 0);
    assert_eq!(compiler.num_exprs, 0);
    assert_eq!(compiler.size_limit, 10 * (1 << 20));
    assert!(compiler.utf8_seqs.is_some());
}

#[test]
fn test_program_new() {
    let program = Program::new();
    assert_eq!(program.insts.len(), 0);
    assert_eq!(program.matches.len(), 0);
    assert_eq!(program.captures.len(), 0);
    assert_eq!(program.start, 0);
    assert!(program.only_utf8);
    assert!(!program.is_bytes);
}

#[test]
fn test_byte_class_set_new() {
    let byte_class_set = ByteClassSet::new();
    assert_eq!(byte_class_set.0.len(), 256);
    for &b in &byte_class_set.0 {
        assert!(!b);
    }
}

#[test]
fn test_suffix_cache_new() {
    let suffix_cache = SuffixCache::new(1000);
    assert_eq!(suffix_cache.table.len(), 1000);
    assert_eq!(suffix_cache.version, 0);
}

