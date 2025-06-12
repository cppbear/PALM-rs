// Answer 0

#[test]
fn test_suffixes_concat_empty() {
    use hir::{Hir, HirKind, Literal, Class};

    // Create an empty Hir for the empty Concat case
    let empty_hir = Hir { kind: HirKind::Concat(vec![]), info: Default::default() };
    let mut lits = Literals::empty();

    // Call the suffixes function with the empty Hir
    suffixes(&empty_hir, &mut lits);

    // Check that no literals were added to the lits
    assert!(lits.is_empty());
}

#[test]
fn test_suffixes_concat_single_empty() {
    use hir::{Hir, HirKind, Literal, Class};

    // Create a single entry Concat with one empty literal
    let single_empty_literal_hir = Hir { 
        kind: HirKind::Concat(vec![Hir { kind: HirKind::Literal(Literal::empty()), info: Default::default() }]), 
        info: Default::default() 
    };
    let mut lits = Literals::empty();

    // Call the suffixes function with a Concat having one empty literal
    suffixes(&single_empty_literal_hir, &mut lits);

    // Check that no literals were added to the lits
    assert!(lits.is_empty());
}

#[test]
fn test_suffixes_concat_multiple_empty() {
    use hir::{Hir, HirKind, Literal, Class};

    // Create multiple empty literals in a Concat
    let multiple_empty_literals_hir = Hir {
        kind: HirKind::Concat(vec![
            Hir { kind: HirKind::Literal(Literal::empty()), info: Default::default() },
            Hir { kind: HirKind::Literal(Literal::empty()), info: Default::default() },
        ]),
        info: Default::default(),
    };
    let mut lits = Literals::empty();

    // Call the suffixes function with the multiple empty literals
    suffixes(&multiple_empty_literals_hir, &mut lits);

    // Check that no literals were added to the lits
    assert!(lits.is_empty());
}

