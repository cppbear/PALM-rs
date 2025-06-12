// Answer 0

#[test]
fn test_c_class_single_char() {
    struct DummyHir {
        start: char,
        end: char,
    }
    impl hir::ClassUnicodeRange for DummyHir {
        fn start(&self) -> char {
            self.start
        }
        fn end(&self) -> char {
            self.end
        }
    }

    let mut compiler = Compiler::new();
    let ranges = vec![DummyHir { start: 'a', end: 'a' }];
    let result = compiler.c_class(&ranges);
    assert!(result.is_ok());
    if let Ok(patch) = result {
        assert_eq!(patch.hole, Hole::One(0));
    }
}

#[test]
fn test_c_class_multiple_chars() {
    struct DummyHir {
        start: char,
        end: char,
    }
    impl hir::ClassUnicodeRange for DummyHir {
        fn start(&self) -> char {
            self.start
        }
        fn end(&self) -> char {
            self.end
        }
    }

    let mut compiler = Compiler::new();
    let ranges = vec![
        DummyHir { start: 'a', end: 'b' },
        DummyHir { start: 'c', end: 'd' },
    ];
    let result = compiler.c_class(&ranges);
    assert!(result.is_ok());
    if let Ok(patch) = result {
        assert_eq!(patch.hole, Hole::One(0));
    }
}

#[test]
fn test_c_class_empty_ranges() {
    let mut compiler = Compiler::new();
    let ranges: Vec<DummyHir> = vec![];
    assert!(compiler.c_class(&ranges).is_err());
}

