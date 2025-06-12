// Answer 0

#[test]
fn test_add_byte_class_when_class_exceeds_limits() {
    struct DummyHir;

    impl Hir for DummyHir {
        fn kind(&self) -> HirKind {
            unimplemented!()
        }
    }

    let mut literals = Literals {
        lits: vec![Literal::empty()],
        limit_size: 5,
        limit_class: 2,
    };
    
    let byte_class = ClassBytes::new(vec![ClassBytesRange { start: 0, end: 255 }]); // Exceeds limit class 

    assert_eq!(literals.add_byte_class(&byte_class), false);
}

#[test]
fn test_add_byte_class_empty_case() {
    struct DummyHir;

    impl Hir for DummyHir {
        fn kind(&self) -> HirKind {
            unimplemented!()
        }
    }

    let mut literals = Literals {
        lits: vec![],
        limit_size: 10,
        limit_class: 10,
    };
    
    let byte_class = ClassBytes::new(vec![ClassBytesRange { start: 1, end: 2 }]); // Within limits 

    assert_eq!(literals.add_byte_class(&byte_class), true);
    assert_eq!(literals.lits.len(), 2); // Expect two literals based on byte class
}

#[test]
fn test_add_byte_class_with_full_bytes() {
    struct DummyHir;

    impl Hir for DummyHir {
        fn kind(&self) -> HirKind {
            unimplemented!()
        }
    }

    let mut literals = Literals {
        lits: vec![Literal { v: vec![1], cut: false }],
        limit_size: 3,
        limit_class: 5,
    };
    
    let byte_class = ClassBytes::new(vec![ClassBytesRange { start: 0, end: 1 }]); // Should be within limits 

    assert_eq!(literals.add_byte_class(&byte_class), true);
    assert_eq!(literals.lits.len(), 4); // Includes literals with appended bytes.
}

