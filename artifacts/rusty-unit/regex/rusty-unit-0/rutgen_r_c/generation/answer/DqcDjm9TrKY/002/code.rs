// Answer 0

fn test_c_repeat_zero_or_one() {
    struct MockHir;
    
    impl MockHir {
        fn new() -> Self {
            MockHir {}
        }
    }

    let mut compiler = Compiler::new();
    let repetition = hir::Repetition {
        kind: hir::RepetitionKind::ZeroOrOne,
        hir: MockHir::new(),
        greedy: true,
    };

    let result = compiler.c_repeat(&repetition);
    assert!(result.is_ok());
}

fn test_c_repeat_zero_or_more() {
    struct MockHir;

    impl MockHir {
        fn new() -> Self {
            MockHir {}
        }
    }

    let mut compiler = Compiler::new();
    let repetition = hir::Repetition {
        kind: hir::RepetitionKind::ZeroOrMore,
        hir: MockHir::new(),
        greedy: true,
    };

    let result = compiler.c_repeat(&repetition);
    assert!(result.is_ok());
}

fn test_c_repeat_at_least() {
    struct MockHir;

    impl MockHir {
        fn new() -> Self {
            MockHir {}
        }
    }

    let mut compiler = Compiler::new();
    let repetition = hir::Repetition {
        kind: hir::RepetitionKind::AtLeast(3),
        hir: MockHir::new(),
        greedy: true,
    };

    let result = compiler.c_repeat(&repetition);
    assert!(result.is_ok());
}

fn test_c_repeat_bounded() {
    struct MockHir;

    impl MockHir {
        fn new() -> Self {
            MockHir {}
        }
    }

    let mut compiler = Compiler::new();
    let repetition = hir::Repetition {
        kind: hir::RepetitionKind::Bounded(2, 5),
        hir: MockHir::new(),
        greedy: true,
    };

    let result = compiler.c_repeat(&repetition);
    assert!(result.is_ok());
}

fn test_c_repeat_exactly() {
    struct MockHir;

    impl MockHir {
        fn new() -> Self {
            MockHir {}
        }
    }

    let mut compiler = Compiler::new();
    let repetition = hir::Repetition {
        kind: hir::RepetitionKind::Exactly(4),
        hir: MockHir::new(),
        greedy: true,
    };

    let result = compiler.c_repeat(&repetition);
    assert!(result.is_ok());
}

