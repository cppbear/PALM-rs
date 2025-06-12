// Answer 0

#[test]
fn test_c_utf8_seq_with_valid_ranges() {
    let mut compiler = Compiler::new();
    let byte_range = vec![Utf8Range { start: 1, end: 2 }, Utf8Range { start: 3, end: 4 }];
    let mut compile_class = CompileClass { c: &mut compiler, ranges: &[] };
    compile_class.c_utf8_seq_(&byte_range.iter()).unwrap();
}

#[test]
fn test_c_utf8_seq_with_cached_entry() {
    let mut compiler = Compiler::new();
    let byte_range = vec![Utf8Range { start: 5, end: 6 }];
    
    // Prime the cache
    let key = SuffixCacheKey { from_inst: ::std::usize::MAX, start: 5, end: 6 };
    compiler.suffix_cache.table.push(SuffixCacheEntry {
        key: key,
        pc: 0,
        version: 0,
    });

    let mut compile_class = CompileClass { c: &mut compiler, ranges: &[] };
    compile_class.c_utf8_seq_(&byte_range.iter()).unwrap();
}

#[test]
fn test_c_utf8_seq_with_empty_ranges() {
    let mut compiler = Compiler::new();
    let byte_range: Vec<Utf8Range> = vec![];
    let mut compile_class = CompileClass { c: &mut compiler, ranges: &[] };
    let result = compile_class.c_utf8_seq_(&byte_range.iter());
    assert!(result.is_ok());
}

#[test]
#[should_panic]
fn test_c_utf8_seq_from_inst_max() {
    let mut compiler = Compiler::new();
    let byte_range = vec![Utf8Range { start: 1, end: 2 }];
    
    // Set insts length to be at max
    compiler.insts.push(MaybeInst::Compiled(Inst::Match(0)));
    let mut compile_class = CompileClass { c: &mut compiler, ranges: &[] };
    
    // This should panic, from_inst would be max and can't decrement
    compile_class.c_utf8_seq_(&byte_range.iter()).unwrap();
}

#[test]
fn test_c_utf8_seq_with_non_zero_length() {
    let mut compiler = Compiler::new();
    let byte_range = vec![Utf8Range { start: 7, end: 8 }];
    
    // Push a compiled instruction to ensure from_inst is not usize::MAX
    compiler.push_compiled(Inst::Match(0));
    
    let mut compile_class = CompileClass { c: &mut compiler, ranges: &[] };
    compile_class.c_utf8_seq_(&byte_range.iter()).unwrap();
}

#[test]
fn test_c_utf8_seq_multiple_ranges() {
    let mut compiler = Compiler::new();
    let byte_ranges = vec![Utf8Range { start: 10, end: 15 }, Utf8Range { start: 20, end: 25 }];
    
    // Push a compiled instruction to ensure from_inst is not usize::MAX
    compiler.push_compiled(Inst::Match(0));
    
    let mut compile_class = CompileClass { c: &mut compiler, ranges: &[] };
    compile_class.c_utf8_seq_(&byte_ranges.iter()).unwrap();
}

