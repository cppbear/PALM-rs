// Answer 0

#[test]
fn test_c_utf8_seq_empty() {
    let mut compiler = Compiler::new();
    let seq: Vec<Utf8Range> = vec![];
    compiler.c_utf8_seq_(seq.iter());
}

#[test]
fn test_c_utf8_seq_single_range() {
    let mut compiler = Compiler::new();
    let seq = vec![Utf8Range { start: 1, end: 3 }];
    compiler.c_utf8_seq_(seq.iter());
}

#[test]
fn test_c_utf8_seq_multiple_ranges() {
    let mut compiler = Compiler::new();
    let seq = vec![
        Utf8Range { start: 10, end: 10 },
        Utf8Range { start: 20, end: 20 },
        Utf8Range { start: 30, end: 50 },
    ];
    compiler.c_utf8_seq_(seq.iter());
}

#[test]
fn test_c_utf8_seq_full_range() {
    let mut compiler = Compiler::new();
    let seq = vec![Utf8Range { start: 0, end: 255 }];
    compiler.c_utf8_seq_(seq.iter());
}

#[test]
fn test_c_utf8_seq_overlapping_ranges() {
    let mut compiler = Compiler::new();
    let seq = vec![
        Utf8Range { start: 5, end: 15 },
        Utf8Range { start: 10, end: 20 },
        Utf8Range { start: 15, end: 25 },
    ];
    compiler.c_utf8_seq_(seq.iter());
}


