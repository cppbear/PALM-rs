// Answer 0

#[test]
fn test_suffixes_empty_hir() {
    struct EmptyHir;
    
    impl EmptyHir {
        fn new() -> Self {
            EmptyHir
        }
    }
    
    let expr = EmptyHir::new();
    let result = suffixes(&expr);
    let expected = Literals::empty();
    assert_eq!(result, expected);
}

#[test]
fn test_suffixes_single_literal() {
    struct SingleLiteralHir;
    
    impl SingleLiteralHir {
        fn new() -> Self {
            SingleLiteralHir
        }
    }
    
    let expr = SingleLiteralHir::new();
    let result = suffixes(&expr);
    let expected = Literals::from(vec!["literal1"]);
    assert_eq!(result, expected);
}

#[test]
fn test_suffixes_complex_hir() {
    struct ComplexHir;
    
    impl ComplexHir {
        fn new() -> Self {
            ComplexHir
        }
    }
    
    let expr = ComplexHir::new();
    let result = suffixes(&expr);
    let expected = Literals::from(vec!["literal1", "literal2"]);
    assert_eq!(result, expected);
}

#[should_panic]
fn test_suffixes_invalid_hir() {
    struct InvalidHir;
    
    impl InvalidHir {
        fn new() -> Self {
            InvalidHir
        }
    }
    
    let expr = InvalidHir::new();
    suffixes(&expr);
}

