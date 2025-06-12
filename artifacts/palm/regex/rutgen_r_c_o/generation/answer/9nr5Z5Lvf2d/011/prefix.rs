// Answer 0

#[test]
fn test_compile_with_valid_utf8_sequence() {
    let mut compiler = Compiler::new();
    let ranges = vec![
        hir::ClassUnicodeRange::new(0x41, 0x5A), // A-Z
    ];
    let mut compile_class = CompileClass { c: &mut compiler, ranges: &ranges };
    let utf8_sequences = Utf8Sequences::new(0x41, 0x5A); // A-Z

    compiler.utf8_seqs = Some(utf8_sequences);
    let _ = compile_class.compile();
}

#[test]
fn test_compile_with_multiple_ranges() {
    let mut compiler = Compiler::new();
    let ranges = vec![
        hir::ClassUnicodeRange::new(0x41, 0x5A), // A-Z
        hir::ClassUnicodeRange::new(0x61, 0x7A), // a-z
    ];
    let mut compile_class = CompileClass { c: &mut compiler, ranges: &ranges };
    let utf8_sequences = Utf8Sequences::new(0x41, 0x7A); // A-Z, a-z

    compiler.utf8_seqs = Some(utf8_sequences);
    let _ = compile_class.compile();
}

#[test]
fn test_compile_with_empty_ranges() {
    let mut compiler = Compiler::new();
    let ranges: Vec<hir::ClassUnicodeRange> = vec![];
    let mut compile_class = CompileClass { c: &mut compiler, ranges: &ranges };
    let utf8_sequences = Utf8Sequences::new(0x00, 0x00); // empty

    compiler.utf8_seqs = Some(utf8_sequences);
    let result = compile_class.compile();
    assert!(result.is_err());
}

#[test]
#[should_panic]
fn test_compile_with_no_utf8_sequences() {
    let mut compiler = Compiler::new();
    let ranges = vec![
        hir::ClassUnicodeRange::new(0x41, 0x5A), // A-Z
    ];
    let mut compile_class = CompileClass { c: &mut compiler, ranges: &ranges };
    
    compiler.utf8_seqs = None; // making utf8_seqs None
    let _ = compile_class.compile();
}

#[test]
fn test_compile_with_last_range() {
    let mut compiler = Compiler::new();
    let ranges = vec![
        hir::ClassUnicodeRange::new(0x41, 0x5A), // A-Z
        hir::ClassUnicodeRange::new(0x61, 0x7A), // a-z
    ];
    let mut compile_class = CompileClass { c: &mut compiler, ranges: &ranges };
    let utf8_sequences = Utf8Sequences::new(0x41, 0x7A); // A-Z, a-z

    compiler.utf8_seqs = Some(utf8_sequences);
    let _ = compile_class.compile();
}

