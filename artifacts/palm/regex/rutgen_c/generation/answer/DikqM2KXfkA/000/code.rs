// Answer 0

#[test]
fn test_c_utf8_seq_reverse() {
    struct MockCompiler {
        compiled: Program,
    }
    
    let mut mock_compiler = MockCompiler {
        compiled: Program { is_reverse: true, ..Default::default() },
    };
    
    let ranges = vec![];
    let mut compile_class = CompileClass {
        c: &mut mock_compiler,
        ranges: &ranges,
    };
    
    let utf8_sequence = Utf8Sequence::new(vec![Utf8Range::new(0x20, 0x7F)]); // Example ranges
    let result = compile_class.c_utf8_seq(&utf8_sequence);
    assert!(result.is_ok());
}

#[test]
fn test_c_utf8_seq_normal() {
    struct MockCompiler {
        compiled: Program,
    }
    
    let mut mock_compiler = MockCompiler {
        compiled: Program { is_reverse: false, ..Default::default() },
    };
    
    let ranges = vec![];
    let mut compile_class = CompileClass {
        c: &mut mock_compiler,
        ranges: &ranges,
    };
    
    let utf8_sequence = Utf8Sequence::new(vec![Utf8Range::new(0x20, 0x7F)]); // Example ranges
    let result = compile_class.c_utf8_seq(&utf8_sequence);
    assert!(result.is_ok());
}

