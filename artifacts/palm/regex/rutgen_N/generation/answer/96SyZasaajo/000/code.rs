// Answer 0

#[test]
fn test_compiler_initialization() {
    struct Compiler {
        insts: Vec<u8>,
        compiled: Program,
        capture_name_idx: std::collections::HashMap<String, usize>,
        num_exprs: usize,
        size_limit: usize,
        suffix_cache: SuffixCache,
        utf8_seqs: Option<Utf8Sequences>,
        byte_classes: ByteClassSet,
    }

    struct Program;

    impl Program {
        fn new() -> Self {
            Program
        }
    }

    struct SuffixCache {
        size: usize,
    }

    impl SuffixCache {
        fn new(size: usize) -> Self {
            SuffixCache { size }
        }
    }

    struct Utf8Sequences {
        start: char,
        end: char,
    }

    impl Utf8Sequences {
        fn new(start: char, end: char) -> Self {
            Utf8Sequences { start, end }
        }
    }

    struct ByteClassSet;

    impl ByteClassSet {
        fn new() -> Self {
            ByteClassSet
        }
    }

    let compiler = Compiler {
        insts: vec![],
        compiled: Program::new(),
        capture_name_idx: std::collections::HashMap::new(),
        num_exprs: 0,
        size_limit: 10 * (1 << 20),
        suffix_cache: SuffixCache::new(1000),
        utf8_seqs: Some(Utf8Sequences::new('\x00', '\x00')),
        byte_classes: ByteClassSet::new(),
    };

    assert_eq!(compiler.insts.len(), 0);
    assert_eq!(compiler.num_exprs, 0);
    assert_eq!(compiler.size_limit, 10 * (1 << 20));
    assert_eq!(compiler.capture_name_idx.len(), 0);
    assert!(compiler.utf8_seqs.is_some());
}

