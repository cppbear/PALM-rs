// Answer 0

#[test]
fn test_repeat_range_literals_min_zero() {
    use regex_syntax::hir::{self, Hir};
    use regex_syntax::literals::Literals;

    let e = Hir::literal("a".to_string());
    let mut lits = Literals::new();

    repeat_range_literals(&e, 0, None, true, &mut lits, |h, _| {
        assert!(matches!(h, Hir::Repetition { .. }));
    });
}

#[test]
fn test_repeat_range_literals_min_greater_than_zero() {
    use regex_syntax::hir::{self, Hir};
    use regex_syntax::literals::Literals;

    let e = Hir::literal("a".to_string());
    let mut lits = Literals::new();

    repeat_range_literals(&e, 2, Some(5), true, &mut lits, |h, _| {
        assert!(matches!(h, Hir::Concat(..)));
    });
}

#[test]
fn test_repeat_range_literals_n_less_than_min() {
    use regex_syntax::hir::{self, Hir};
    use regex_syntax::literals::Literals;

    let e = Hir::literal("a".to_string());
    let mut lits = Literals::new();
    lits.limit_size = 1; // Setting limit_size < min

    repeat_range_literals(&e, 2, Some(5), true, &mut lits, |h, _| {
        assert!(matches!(h, Hir::Concat(..)));
    });

    // Check that cut() was called
    assert!(lits.cut_called());
}

#[test]
fn test_repeat_range_literals_max_condition_false() {
    use regex_syntax::hir::{self, Hir};
    use regex_syntax::literals::Literals;

    let e = Hir::literal("a".to_string());
    let mut lits = Literals::new();

    repeat_range_literals(&e, 2, Some(2), true, &mut lits, |h, _| {
        assert!(matches!(h, Hir::Concat(..)));
    });

    // Check that cut() was called
    assert!(lits.cut_called());
}

