// Answer 0

#[test]
fn test_c_utf8_seq_empty_range() {
    let mut compiler = Compiler::new();
    let utf8_range: Vec<Utf8Range> = Vec::new();
    let mut compile_class = CompileClass { c: &mut compiler, ranges: &[] };
    compile_class.c_utf8_seq_(utf8_range);
}

#[test]
fn test_c_utf8_seq_one_byte_range() {
    let mut compiler = Compiler::new();
    let utf8_range: Vec<Utf8Range> = vec![Utf8Range { start: 0, end: 0 }];
    let mut compile_class = CompileClass { c: &mut compiler, ranges: &[] };
    compile_class.c_utf8_seq_(utf8_range);
}

#[test]
fn test_c_utf8_seq_full_byte_range() {
    let mut compiler = Compiler::new();
    let utf8_range: Vec<Utf8Range> = vec![Utf8Range { start: 1, end: 255 }];
    let mut compile_class = CompileClass { c: &mut compiler, ranges: &[] };
    compile_class.c_utf8_seq_(utf8_range);
}

#[test]
fn test_c_utf8_seq_zero_to_full_range() {
    let mut compiler = Compiler::new();
    let utf8_range: Vec<Utf8Range> = vec![Utf8Range { start: 0, end: 255 }];
    let mut compile_class = CompileClass { c: &mut compiler, ranges: &[] };
    compile_class.c_utf8_seq_(utf8_range);
}

#[test]
#[should_panic]
fn test_c_utf8_seq_invalid_high_range() {
    let mut compiler = Compiler::new();
    let utf8_range: Vec<Utf8Range> = vec![Utf8Range { start: 255, end: 256 }];
    let mut compile_class = CompileClass { c: &mut compiler, ranges: &[] };
    compile_class.c_utf8_seq_(utf8_range);
}

