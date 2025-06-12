// Answer 0

fn test_c_utf8_seq_empty() {
    let mut compiler = Compiler::new();
    let mut compile_class = CompileClass {
        c: &mut compiler,
        ranges: &[],
    };
    let seq: Vec<Utf8Range> = vec![];
    
    let result = compile_class.c_utf8_seq_(seq);
    assert!(result.is_ok());

    if let Ok(patch) = result {
        assert_eq!(patch.hole, Hole::None);
        assert_eq!(patch.entry, ::std::usize::MAX - 1);
    }
}

fn test_c_utf8_seq_single_byte() {
    let mut compiler = Compiler::new();
    let mut compile_class = CompileClass {
        c: &mut compiler,
        ranges: &[],
    };
    let seq: Vec<Utf8Range> = vec![Utf8Range { start: 0, end: 0 }];
    
    let result = compile_class.c_utf8_seq_(seq.iter());

    assert!(result.is_ok());

    if let Ok(patch) = result {
        assert_ne!(patch.entry, ::std::usize::MAX);
    }
}

fn test_c_utf8_seq_multiple_bytes() {
    let mut compiler = Compiler::new();
    let mut compile_class = CompileClass {
        c: &mut compiler,
        ranges: &[],
    };
    let seq: Vec<Utf8Range> = vec![
        Utf8Range { start: 0, end: 1 },
        Utf8Range { start: 2, end: 4 },
    ];
    
    let result = compile_class.c_utf8_seq_(seq.iter());

    assert!(result.is_ok());

    if let Ok(patch) = result {
        assert_ne!(patch.entry, ::std::usize::MAX);
    }
}

fn test_c_utf8_seq_overlapping_bytes() {
    let mut compiler = Compiler::new();
    let mut compile_class = CompileClass {
        c: &mut compiler,
        ranges: &[],
    };
    let seq: Vec<Utf8Range> = vec![
        Utf8Range { start: 0, end: 2 },
        Utf8Range { start: 1, end: 3 },
    ];
    
    let result = compile_class.c_utf8_seq_(seq.iter());

    assert!(result.is_ok());

    if let Ok(patch) = result {
        assert_ne!(patch.entry, ::std::usize::MAX);
    }
}

