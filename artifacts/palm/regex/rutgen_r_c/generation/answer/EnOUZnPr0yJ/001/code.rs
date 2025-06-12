// Answer 0

#[test]
#[should_panic(expected = "assertion failed: !ranges.is_empty()")]
fn test_c_class_empty_ranges() {
    let mut compiler = Compiler::new();
    let empty_ranges: Vec<hir::ClassUnicodeRange> = vec![];
    let _ = compiler.c_class(&empty_ranges);
}

#[test]
fn test_c_class_single_character_range() {
    struct DummyHir;
    
    impl hir::ClassUnicodeRange for DummyHir {
        fn start(&self) -> char {
            'a'
        }
        fn end(&self) -> char {
            'a'
        }
    }

    let mut compiler = Compiler::new();
    let ranges = vec![DummyHir];
    let result = compiler.c_class(&ranges);
    assert!(result.is_ok());
}

#[test]
fn test_c_class_multiple_character_ranges() {
    struct DummyHir;
    
    impl hir::ClassUnicodeRange for DummyHir {
        fn start(&self) -> char {
            'a'
        }
        fn end(&self) -> char {
            'z'
        }
    }

    let mut compiler = Compiler::new();
    let ranges = vec![DummyHir];
    let result = compiler.c_class(&ranges);
    assert!(result.is_ok());
}

#[test]
fn test_c_class_byte_usage() {
    struct DummyHir;
    
    impl hir::ClassUnicodeRange for DummyHir {
        fn start(&self) -> char {
            'A'
        }
        fn end(&self) -> char {
            'Z'
        }
    }

    let mut compiler = Compiler::new();
    compiler.bytes(true);
    let ranges = vec![DummyHir];
    let result = compiler.c_class(&ranges);
    assert!(result.is_ok());
}

