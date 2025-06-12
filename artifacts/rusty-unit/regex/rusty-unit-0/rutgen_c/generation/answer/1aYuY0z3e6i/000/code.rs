// Answer 0

#[test]
fn test_c_utf8_seq_with_empty_sequence() {
    let mut compiler = Compiler::new();
    let seq: Vec<Utf8Range> = Vec::new();
    let mut compile_class = CompileClass { c: &mut compiler, ranges: &[] };
    
    let result = compile_class.c_utf8_seq_(seq);
    assert!(result.is_ok());
}

#[test]
fn test_c_utf8_seq_with_one_range() {
    let mut compiler = Compiler::new();
    let seq = vec![Utf8Range { start: 0x61, end: 0x61 }]; // 'a' character
    let mut compile_class = CompileClass { c: &mut compiler, ranges: &[] };
    
    let result = compile_class.c_utf8_seq_(seq);
    assert!(result.is_ok());
}

#[test]
fn test_c_utf8_seq_with_multiple_ranges() {
    let mut compiler = Compiler::new();
    let seq = vec![
        Utf8Range { start: 0x61, end: 0x61 }, // 'a'
        Utf8Range { start: 0x62, end: 0x62 }, // 'b'
        Utf8Range { start: 0x63, end: 0x63 }, // 'c'
    ];
    let mut compile_class = CompileClass { c: &mut compiler, ranges: &[] };
    
    let result = compile_class.c_utf8_seq_(seq);
    assert!(result.is_ok());
}

#[test]
fn test_c_utf8_seq_with_overlapping_ranges() {
    let mut compiler = Compiler::new();
    let seq = vec![
        Utf8Range { start: 0x61, end: 0x63 }, // 'a' to 'c'
        Utf8Range { start: 0x62, end: 0x64 }, // 'b' to 'd'
    ];
    let mut compile_class = CompileClass { c: &mut compiler, ranges: &[] };
    
    let result = compile_class.c_utf8_seq_(seq);
    assert!(result.is_ok());
}

