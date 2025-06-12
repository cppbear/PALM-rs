// Answer 0

#[test]
fn test_c_repetition_zero_or_one() {
    let mut compiler = Compiler::new().size_limit(1024);
    let expr = Hir::new_repetition(hir::Repetition {
        kind: hir::RepetitionKind::ZeroOrOne,
        greedy: true,
        hir: Hir::new_literal(hir::Literal::Unicode('a')),
    });
    let _ = compiler.c(&expr);
}

#[test]
fn test_c_repetition_zero_or_more() {
    let mut compiler = Compiler::new().size_limit(2048);
    let expr = Hir::new_repetition(hir::Repetition {
        kind: hir::RepetitionKind::ZeroOrMore,
        greedy: true,
        hir: Hir::new_literal(hir::Literal::Unicode('b')),
    });
    let _ = compiler.c(&expr);
}

#[test]
fn test_c_repetition_one_or_more() {
    let mut compiler = Compiler::new().size_limit(4096);
    let expr = Hir::new_repetition(hir::Repetition {
        kind: hir::RepetitionKind::OneOrMore,
        greedy: true,
        hir: Hir::new_literal(hir::Literal::Unicode('c')),
    });
    let _ = compiler.c(&expr);
}

#[test]
fn test_c_repetition_range_at_least() {
    let mut compiler = Compiler::new().size_limit(8192);
    let expr = Hir::new_repetition(hir::Repetition {
        kind: hir::RepetitionKind::Range(hir::RepetitionRange::AtLeast(1)),
        greedy: true,
        hir: Hir::new_literal(hir::Literal::Unicode('d')),
    });
    let _ = compiler.c(&expr);
}

#[test]
fn test_c_repetition_range_bounded() {
    let mut compiler = Compiler::new().size_limit(16384);
    let expr = Hir::new_repetition(hir::Repetition {
        kind: hir::RepetitionKind::Range(hir::RepetitionRange::Bounded(1, 3)),
        greedy: true,
        hir: Hir::new_literal(hir::Literal::Unicode('e')),
    });
    let _ = compiler.c(&expr);
}

