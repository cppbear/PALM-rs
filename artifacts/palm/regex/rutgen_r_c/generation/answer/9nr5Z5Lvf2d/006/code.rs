// Answer 0

fn test_compile_with_empty_utf8_sequences() {
    struct MockCompiler {
        utf8_seqs: Option<Utf8Sequences>,
        suffix_cache: SuffixCache,
        insts: Vec<MaybeInst>,
    }

    impl MockCompiler {
        fn new() -> Self {
            Self {
                utf8_seqs: None,
                suffix_cache: SuffixCache::new(1000),
                insts: vec![],
            }
        }
    }

    let mut compiler = MockCompiler::new();
    let ranges = vec![];

    let compile_class = CompileClass {
        c: &mut compiler,
        ranges: &ranges,
    };

    let result = compile_class.compile();
    assert!(result.is_err());
}

fn test_compile_with_single_utf8_sequence() {
    struct MockCompiler {
        utf8_seqs: Option<Utf8Sequences>,
        suffix_cache: SuffixCache,
        insts: Vec<MaybeInst>,
    }
    
    impl MockCompiler {
        fn new() -> Self {
            Self {
                utf8_seqs: Some(Utf8Sequences::new('\x00', '\x00')),
                suffix_cache: SuffixCache::new(1000),
                insts: vec![],
            }
        }
    }

    let mut compiler = MockCompiler::new();
    let ranges = vec![hir::ClassUnicodeRange::new(0, 1)];

    let compile_class = CompileClass {
        c: &mut compiler,
        ranges: &ranges,
    };

    let result = compile_class.compile();
    assert!(result.is_ok());
}

fn test_compile_with_multiple_utf8_sequences() {
    struct MockCompiler {
        utf8_seqs: Option<Utf8Sequences>,
        suffix_cache: SuffixCache,
        insts: Vec<MaybeInst>,
    }
    
    impl MockCompiler {
        fn new() -> Self {
            Self {
                utf8_seqs: Some(Utf8Sequences::new('\x00', '\x01')),
                suffix_cache: SuffixCache::new(1000),
                insts: vec![],
            }
        }
    }

    let mut compiler = MockCompiler::new();
    let ranges = vec![
        hir::ClassUnicodeRange::new(1, 2),
        hir::ClassUnicodeRange::new(2, 3),
    ];

    let compile_class = CompileClass {
        c: &mut compiler,
        ranges: &ranges,
    };

    let result = compile_class.compile();
    assert!(result.is_ok());
}

fn test_compile_with_utf8_seq_error() {
    struct MockCompiler {
        utf8_seqs: Option<Utf8Sequences>,
        suffix_cache: SuffixCache,
        insts: Vec<MaybeInst>,
    }
    
    impl MockCompiler {
        fn new() -> Self {
            Self {
                utf8_seqs: Some(Utf8Sequences::new('\x00', '\x00')),
                suffix_cache: SuffixCache::new(1000),
                insts: vec![],
            }
        }

        fn c_utf8_seq(&mut self, _seq: &Utf8Sequence) -> Result {
            Err(Error::Syntax(String::from("Invalid UTF8 sequence")))
        }
    }

    let mut compiler = MockCompiler::new();
    let ranges = vec![hir::ClassUnicodeRange::new(0, 1)];

    let compile_class = CompileClass {
        c: &mut compiler,
        ranges: &ranges,
    };

    let result = compile_class.compile();
    assert!(result.is_err());    
}

