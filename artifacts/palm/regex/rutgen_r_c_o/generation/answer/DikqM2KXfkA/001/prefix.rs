// Answer 0

#[test]
fn test_c_utf8_seq_reverse_true_single_range() {
    let mut compiler = Compiler {
        insts: vec![],
        compiled: Program { is_reverse: true },
        capture_name_idx: HashMap::new(),
        num_exprs: 0,
        size_limit: 100,
        suffix_cache: SuffixCache::default(),
        utf8_seqs: None,
        byte_classes: ByteClassSet::default(),
    };
    let range = Utf8Range { start: 0x0000, end: 0x007F };
    let utf8_sequence = Utf8Sequence::from(vec![range]);
    let mut compile_class = CompileClass { c: &mut compiler, ranges: &[] };
    let _ = compile_class.c_utf8_seq(&utf8_sequence);
}

#[test]
fn test_c_utf8_seq_reverse_true_multiple_ranges() {
    let mut compiler = Compiler {
        insts: vec![],
        compiled: Program { is_reverse: true },
        capture_name_idx: HashMap::new(),
        num_exprs: 0,
        size_limit: 100,
        suffix_cache: SuffixCache::default(),
        utf8_seqs: None,
        byte_classes: ByteClassSet::default(),
    };
    let ranges = vec![Utf8Range { start: 0x0000, end: 0x007F },
                     Utf8Range { start: 0x0080, end: 0x07FF },
                     Utf8Range { start: 0x0800, end: 0xFFFF }];
    let utf8_sequence = Utf8Sequence::from(ranges);
    let mut compile_class = CompileClass { c: &mut compiler, ranges: &[] };
    let _ = compile_class.c_utf8_seq(&utf8_sequence);
}

#[test]
fn test_c_utf8_seq_reverse_true_max_ranges() {
    let mut compiler = Compiler {
        insts: vec![],
        compiled: Program { is_reverse: true },
        capture_name_idx: HashMap::new(),
        num_exprs: 0,
        size_limit: 100,
        suffix_cache: SuffixCache::default(),
        utf8_seqs: None,
        byte_classes: ByteClassSet::default(),
    };
    let ranges = vec![Utf8Range { start: 0x0000, end: 0x007F },
                     Utf8Range { start: 0x0080, end: 0x07FF },
                     Utf8Range { start: 0x0800, end: 0xFFFF },
                     Utf8Range { start: 0x1000, end: 0x10FFFF }];
    let utf8_sequence = Utf8Sequence::from(ranges);
    let mut compile_class = CompileClass { c: &mut compiler, ranges: &[] };
    let _ = compile_class.c_utf8_seq(&utf8_sequence);
}

#[test]
fn test_c_utf8_seq_reverse_true_empty_sequence() {
    let mut compiler = Compiler {
        insts: vec![],
        compiled: Program { is_reverse: true },
        capture_name_idx: HashMap::new(),
        num_exprs: 0,
        size_limit: 100,
        suffix_cache: SuffixCache::default(),
        utf8_seqs: None,
        byte_classes: ByteClassSet::default(),
    };
    let utf8_sequence = Utf8Sequence::from(vec![]);
    let mut compile_class = CompileClass { c: &mut compiler, ranges: &[] };
    let _ = compile_class.c_utf8_seq(&utf8_sequence);
}

#[test]
fn test_c_utf8_seq_reverse_true_large_range() {
    let mut compiler = Compiler {
        insts: vec![],
        compiled: Program { is_reverse: true },
        capture_name_idx: HashMap::new(),
        num_exprs: 0,
        size_limit: 100,
        suffix_cache: SuffixCache::default(),
        utf8_seqs: None,
        byte_classes: ByteClassSet::default(),
    };
    let range = Utf8Range { start: 0x0000, end: 0x10FFFF };
    let utf8_sequence = Utf8Sequence::from(vec![range]);
    let mut compile_class = CompileClass { c: &mut compiler, ranges: &[] };
    let _ = compile_class.c_utf8_seq(&utf8_sequence);
}

