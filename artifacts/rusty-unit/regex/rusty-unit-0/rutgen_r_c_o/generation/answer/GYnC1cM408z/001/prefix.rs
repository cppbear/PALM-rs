// Answer 0

#[test]
fn test_compile_many_edge_case_1() {
    let exprs = vec![
        Hir::repetition(hir::Repetition {
            kind: hir::RepetitionKind::ZeroOrMore,
            greedy: true,
            hir: Box::new(Hir::char('a')),
        }),
        Hir::repetition(hir::Repetition {
            kind: hir::RepetitionKind::ZeroOrMore,
            greedy: true,
            hir: Box::new(Hir::char('b')),
        }),
    ];
    let compiler = Compiler::new().dfa(true);
    let _ = compiler.compile_many(&exprs);
}

#[test]
fn test_compile_many_edge_case_2() {
    let exprs = (0..1000).map(|i| {
        Hir::repetition(hir::Repetition {
            kind: hir::RepetitionKind::ZeroOrMore,
            greedy: true,
            hir: Box::new(Hir::char('x' as char + i as char)),
        })
    }).collect::<Vec<_>>();
    let compiler = Compiler::new().dfa(true);
    let _ = compiler.compile_many(&exprs);
}

#[test]
fn test_compile_many_edge_case_3() {
    let exprs = vec![
        Hir::repetition(hir::Repetition {
            kind: hir::RepetitionKind::ZeroOrMore,
            greedy: true,
            hir: Box::new(Hir::char('c')),
        }),
        Hir::repetition(hir::Repetition {
            kind: hir::RepetitionKind::ZeroOrMore,
            greedy: true,
            hir: Box::new(Hir::char('d')),
        }),
        Hir::repetition(hir::Repetition {
            kind: hir::RepetitionKind::ZeroOrMore,
            greedy: true,
            hir: Box::new(Hir::char('e')),
        }),
    ];
    let compiler = Compiler::new().dfa(true);
    let _ = compiler.compile_many(&exprs);
} 

#[test]
fn test_compile_many_edge_case_4() {
    let exprs = vec![
        Hir::repetition(hir::Repetition {
            kind: hir::RepetitionKind::ZeroOrMore,
            greedy: true,
            hir: Box::new(Hir::char('m')),
        }),
        Hir::repetition(hir::Repetition {
            kind: hir::RepetitionKind::ZeroOrMore,
            greedy: true,
            hir: Box::new(Hir::char('n')),
        }),
        Hir::repetition(hir::Repetition {
            kind: hir::RepetitionKind::ZeroOrMore,
            greedy: true,
            hir: Box::new(Hir::char('o')),
        }),
        Hir::repetition(hir::Repetition {
            kind: hir::RepetitionKind::ZeroOrMore,
            greedy: true,
            hir: Box::new(Hir::char('p')),
        }),
    ];
    let compiler = Compiler::new().dfa(true);
    let _ = compiler.compile_many(&exprs);
} 

#[test]
fn test_compile_many_edge_case_5() {
    let exprs = vec![
        Hir::repetition(hir::Repetition {
            kind: hir::RepetitionKind::ZeroOrMore,
            greedy: true,
            hir: Box::new(Hir::char('q')),
        }),
        Hir::repetition(hir::Repetition {
            kind: hir::RepetitionKind::ZeroOrMore,
            greedy: true,
            hir: Box::new(Hir::char('r')),
        }),
        Hir::repetition(hir::Repetition {
            kind: hir::RepetitionKind::ZeroOrMore,
            greedy: true,
            hir: Box::new(Hir::char('s')),
        }),
        Hir::repetition(hir::Repetition {
            kind: hir::RepetitionKind::ZeroOrMore,
            greedy: true,
            hir: Box::new(Hir::char('t')),
        }),
        Hir::repetition(hir::Repetition {
            kind: hir::RepetitionKind::ZeroOrMore,
            greedy: true,
            hir: Box::new(Hir::char('u')),
        }),
    ];
    let compiler = Compiler::new().dfa(true);
    let _ = compiler.compile_many(&exprs);
} 

