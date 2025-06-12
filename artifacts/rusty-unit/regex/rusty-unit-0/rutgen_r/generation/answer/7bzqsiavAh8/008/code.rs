// Answer 0

#[test]
fn test_repeat_range_literals_min_zero() {
    use regex_syntax::hir::{Hir, Literals, Repetition, RepetitionKind};
    use std::cmp;
    
    let e = Hir::literal("a".to_string());
    let mut lits = Literals::new();

    let mut f = |hir: &Hir, _: &mut Literals| {
        assert_eq!(hir, &Hir::repetition(Repetition {
            kind: RepetitionKind::ZeroOrMore,
            greedy: true,
            hir: Box::new(e.clone()),
        }));
    };

    repeat_range_literals(&e, 0, None, true, &mut lits, f);
}

#[test]
fn test_repeat_range_literals_min_greater_than_zero() {
    use regex_syntax::hir::{Hir, Literals};
    use std::cmp;
    use std::iter;

    let e = Hir::literal("b".to_string());
    let mut lits = Literals::new();
    lits.limit_size = 5; // Set a limit size for the Literals

    let mut f = |hir: &Hir, _: &mut Literals| {
        assert_eq!(hir, &Hir::concat(vec![e.clone(), e.clone(), e.clone()]));
    };

    repeat_range_literals(&e, 3, None, true, &mut lits, f);
}

#[test]
fn test_repeat_range_literals_with_max() {
    use regex_syntax::hir::{Hir, Literals};
    
    let e = Hir::literal("c".to_string());
    let mut lits = Literals::new();
    lits.limit_size = 2;

    let mut f = |hir: &Hir, _: &mut Literals| {
        assert_eq!(hir, &Hir::concat(vec![e.clone(), e.clone()]));
    };

    repeat_range_literals(&e, 2, Some(5), true, &mut lits, f);
}

