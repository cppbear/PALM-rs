// Answer 0

#[test]
fn test_c_utf8_seq_simple() {
    let mut compiler = Compiler::new();
    let mut suffix_cache = SuffixCache::new(1000);
    suffix_cache.version = 1; // to ensure our key is cached
    compiler.suffix_cache = suffix_cache;

    let byte_range = Utf8Range { start: 1, end: 10 };
    let seq = vec![&byte_range];
    
    let mut compile_class = CompileClass { c: &mut compiler, ranges: &[] };
    let _ = compile_class.c_utf8_seq_(seq);
}

#[test]
fn test_c_utf8_seq_multiple_ranges() {
    let mut compiler = Compiler::new();
    let mut suffix_cache = SuffixCache::new(1000);
    suffix_cache.version = 1; // version to match our key checks
    compiler.suffix_cache = suffix_cache;

    let byte_ranges = vec![
        Utf8Range { start: 1, end: 5 },
        Utf8Range { start: 6, end: 10 },
        Utf8Range { start: 11, end: 255 },
    ];
    
    let seq = byte_ranges.iter().collect::<Vec<_>>();
    
    let mut compile_class = CompileClass { c: &mut compiler, ranges: &[] };
    let _ = compile_class.c_utf8_seq_(seq);
}

#[test]
fn test_c_utf8_seq_cache_hit() {
    let mut compiler = Compiler::new();
    let mut suffix_cache = SuffixCache::new(1000);
    suffix_cache.version = 1;
    compiler.suffix_cache = suffix_cache;

    // Initially caching a pc
    let key = SuffixCacheKey { from_inst: 0, start: 1, end: 5 };
    compiler.suffix_cache.table.push(SuffixCacheEntry { key, pc: 0, version: 1 }); 

    let byte_range = Utf8Range { start: 1, end: 5 };
    let seq = vec![&byte_range];

    let mut compile_class = CompileClass { c: &mut compiler, ranges: &[] };
    let _ = compile_class.c_utf8_seq_(seq);
}

#[test]
fn test_c_utf8_seq_with_holes() {
    let mut compiler = Compiler::new();
    let mut suffix_cache = SuffixCache::new(1000);
    suffix_cache.version = 1;
    compiler.suffix_cache = suffix_cache;

    let byte_range = Utf8Range { start: 50, end: 100 };
    let seq = vec![&byte_range];

    // Manually push an instruction to ensure from_inst is not usize::MAX
    compiler.push_compiled(Inst::Match(0));

    let mut compile_class = CompileClass { c: &mut compiler, ranges: &[] };
    let _ = compile_class.c_utf8_seq_(seq);
}

#[test]
fn test_c_utf8_seq_edge_case() {
    let mut compiler = Compiler::new();
    let mut suffix_cache = SuffixCache::new(1000);
    suffix_cache.version = 1;
    compiler.suffix_cache = suffix_cache;

    let byte_range = Utf8Range { start: 255, end: 255 };
    let seq = vec![&byte_range];

    // Manually push an instruction to ensure valid from_inst state
    compiler.push_compiled(Inst::Match(0));

    let mut compile_class = CompileClass { c: &mut compiler, ranges: &[] };
    let _ = compile_class.c_utf8_seq_(seq);
}

