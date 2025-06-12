// Answer 0

#[test]
fn test_c_utf8_seq_empty() {
    let mut compiler = Compiler::new();
    let mut compile_class = CompileClass {
        c: &mut compiler,
        ranges: &[],
    };
    
    let result = compile_class.c_utf8_seq_(iter::empty::<Utf8Range>());
    assert!(result.is_ok());
}

#[test]
fn test_c_utf8_seq_single_range() {
    let mut compiler = Compiler::new();
    let mut compile_class = CompileClass {
        c: &mut compiler,
        ranges: &[],
    };

    let byte_ranges = vec![Utf8Range::new(0, 5)];
    let result = compile_class.c_utf8_seq_(byte_ranges.iter());
    assert!(result.is_ok());
}

#[test]
fn test_c_utf8_seq_multiple_ranges() {
    let mut compiler = Compiler::new();
    let mut compile_class = CompileClass {
        c: &mut compiler,
        ranges: &[],
    };

    let byte_ranges = vec![
        Utf8Range::new(0, 5),
        Utf8Range::new(6, 10),
    ];
    let result = compile_class.c_utf8_seq_(byte_ranges.iter());
    assert!(result.is_ok());
}

#[test]
fn test_c_utf8_seq_cache_hit() {
    let mut compiler = Compiler::new();
    let mut compile_class = CompileClass {
        c: &mut compiler,
        ranges: &[],
    };

    let byte_ranges = vec![Utf8Range::new(0, 5)];
    compile_class.c_utf8_seq_(byte_ranges.iter()).unwrap(); // Fill cache
    let result = compile_class.c_utf8_seq_(byte_ranges.iter());
    assert!(result.is_ok());
}

#[should_panic(expected = "attempt to subtract with overflow")]
#[test]
fn test_c_utf8_seq_underflow() {
    let mut compiler = Compiler::new();
    let mut compile_class = CompileClass {
        c: &mut compiler,
        ranges: &[],
    };

    compiler.insts.push(MaybeInst::Compiled(Inst::Match(0))); // Fill some insts
    let byte_ranges = vec![Utf8Range::new(0, 5)];
    
    // Clear compiled instructions to make the length 0 (Underflow on checked_sub)
    compiler.insts.clear();
    compile_class.c_utf8_seq_(byte_ranges.iter()).unwrap();
}

