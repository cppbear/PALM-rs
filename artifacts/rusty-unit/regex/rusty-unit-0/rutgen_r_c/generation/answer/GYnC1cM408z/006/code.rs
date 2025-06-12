// Answer 0

fn test_compile_many_valid() {
    use syntax::hir::{self, Hir, Repetition, RepetitionKind};

    // Setup Compiler
    let mut compiler = Compiler::new()
        .size_limit(100) // arbitrary size limit
        .dfa(true); // set to true to satisfy needs_dotstar

    // Prepare test inputs
    let exprs = vec![
        Hir::repetition(Repetition {
            kind: RepetitionKind::ZeroOrMore,
            greedy: false,
            hir: Box::new(Hir::any(true)),
        }),
        Hir::repetition(Repetition {
            kind: RepetitionKind::ZeroOrMore,
            greedy: true,
            hir: Box::new(Hir::any(false)),
        }),
    ];

    // Execute the function
    let result = compiler.compile_many(&exprs);

    // Assert the result
    assert!(result.is_ok());

    // Grab the resulting program
    let program = result.unwrap();

    assert!(!program.matches.is_empty());
    assert_eq!(program.is_anchored_start, true);
    assert_eq!(program.is_anchored_end, true);
}

fn test_compile_many_invalid_length() {
    use syntax::hir::{self, Hir, Repetition, RepetitionKind};

    // Setup Compiler
    let mut compiler = Compiler::new().dfa(true);

    // Prepare test inputs
    let exprs: Vec<Hir> = vec![]; // empty input to trigger assert

    // Execute the function and expect panic
    let result = std::panic::catch_unwind(|| {
        compiler.compile_many(&exprs);
    });

    assert!(result.is_err());
}

fn test_compile_many_needs_dotstar_false() {
    use syntax::hir::{self, Hir, Repetition, RepetitionKind};

    // Setup Compiler
    let mut compiler = Compiler::new().dfa(false); // set to false

    // Prepare test inputs
    let exprs = vec![
        Hir::repetition(Repetition {
            kind: RepetitionKind::ZeroOrMore,
            greedy: false,
            hir: Box::new(Hir::any(true)),
        }),
        Hir::repetition(Repetition {
            kind: RepetitionKind::ZeroOrMore,
            greedy: true,
            hir: Box::new(Hir::any(false)),
        }),
    ];

    // Execute the function
    let result = compiler.compile_many(&exprs);

    // Check that it does not need dotstar, the program should be created
    assert!(result.is_ok());
    let program = result.unwrap();
    assert_eq!(program.start, 0);
}

fn test_compile_many_capture_ok() {
    use syntax::hir::{self, Hir, Repetition, RepetitionKind};

    // Setup Compiler
    let mut compiler = Compiler::new().dfa(true);

    // Prepare test inputs
    let exprs = vec![
        Hir::repetition(Repetition {
            kind: RepetitionKind::ZeroOrMore,
            greedy: false,
            hir: Box::new(Hir::any(true)),
        }),
        Hir::repetition(Repetition {
            kind: RepetitionKind::OneOrMore,
            greedy: true,
            hir: Box::new(Hir::any(false)),
        }),
    ];

    // Execute the function
    let result = compiler.compile_many(&exprs);

    // Check that capture works fine
    assert!(result.is_ok());
    let program = result.unwrap();
    assert_eq!(program.matches.len(), exprs.len());
}

