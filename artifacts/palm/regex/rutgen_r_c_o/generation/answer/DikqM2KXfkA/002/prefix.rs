// Answer 0

#[test]
fn test_c_utf8_seq_empty() {
    let mut compiler = Compiler {
        insts: vec![],
        compiled: Program { is_reverse: false },
        capture_name_idx: HashMap::new(),
        num_exprs: 0,
        size_limit: 1000,
        suffix_cache: SuffixCache::new(),
        utf8_seqs: None,
        byte_classes: ByteClassSet::new(),
    };
    
    let ranges: Vec<hir::ClassUnicodeRange> = vec![];
    let utf8_sequence = Utf8Sequence::from(ranges);
    
    let mut compile_class = CompileClass { c: &mut compiler, ranges: &[] };
    compile_class.c_utf8_seq(&utf8_sequence);
}

#[test]
fn test_c_utf8_seq_single_range() {
    let mut compiler = Compiler {
        insts: vec![],
        compiled: Program { is_reverse: false },
        capture_name_idx: HashMap::new(),
        num_exprs: 0,
        size_limit: 1000,
        suffix_cache: SuffixCache::new(),
        utf8_seqs: None,
        byte_classes: ByteClassSet::new(),
    };
    
    let ranges: Vec<hir::ClassUnicodeRange> = vec![hir::ClassUnicodeRange::new(0x0000, 0x007F)];
    let utf8_sequence = Utf8Sequence::from(ranges);
    
    let mut compile_class = CompileClass { c: &mut compiler, ranges: &[] };
    compile_class.c_utf8_seq(&utf8_sequence);
}

#[test]
fn test_c_utf8_seq_multiple_ranges() {
    let mut compiler = Compiler {
        insts: vec![],
        compiled: Program { is_reverse: false },
        capture_name_idx: HashMap::new(),
        num_exprs: 0,
        size_limit: 1000,
        suffix_cache: SuffixCache::new(),
        utf8_seqs: None,
        byte_classes: ByteClassSet::new(),
    };

    let ranges: Vec<hir::ClassUnicodeRange> = vec![
        hir::ClassUnicodeRange::new(0x0000, 0x007F),
        hir::ClassUnicodeRange::new(0x0080, 0x07FF),
        hir::ClassUnicodeRange::new(0x0800, 0xFFFF)
    ];
    let utf8_sequence = Utf8Sequence::from(ranges);
    
    let mut compile_class = CompileClass { c: &mut compiler, ranges: &[] };
    compile_class.c_utf8_seq(&utf8_sequence);
}

#[test]
fn test_c_utf8_seq_large_sequence() {
    let mut compiler = Compiler {
        insts: vec![],
        compiled: Program { is_reverse: false },
        capture_name_idx: HashMap::new(),
        num_exprs: 0,
        size_limit: 1000,
        suffix_cache: SuffixCache::new(),
        utf8_seqs: None,
        byte_classes: ByteClassSet::new(),
    };

    let ranges: Vec<hir::ClassUnicodeRange> = (0..1000).map(|i| hir::ClassUnicodeRange::new(i, i)).collect();
    let utf8_sequence = Utf8Sequence::from(ranges);
    
    let mut compile_class = CompileClass { c: &mut compiler, ranges: &[] };
    compile_class.c_utf8_seq(&utf8_sequence);
}

#[test]
fn test_c_utf8_seq_high_range() {
    let mut compiler = Compiler {
        insts: vec![],
        compiled: Program { is_reverse: false },
        capture_name_idx: HashMap::new(),
        num_exprs: 0,
        size_limit: 1000,
        suffix_cache: SuffixCache::new(),
        utf8_seqs: None,
        byte_classes: ByteClassSet::new(),
    };

    let ranges: Vec<hir::ClassUnicodeRange> = vec![hir::ClassUnicodeRange::new(0x10000, 0x10FFFF)];
    let utf8_sequence = Utf8Sequence::from(ranges);
    
    let mut compile_class = CompileClass { c: &mut compiler, ranges: &[] };
    compile_class.c_utf8_seq(&utf8_sequence);
}

