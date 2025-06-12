// Answer 0

#[test]
fn test_suffixes_alternation_with_no_exprs() {
    let expr = Hir::alternation(vec![]);
    let mut lits = Literals::empty();
    suffixes(&expr, &mut lits);
}

#[test]
fn test_suffixes_alternation_with_one_expr() {
    let expr = Hir::alternation(vec![Hir::literal(Literal::Unicode('a'))]);
    let mut lits = Literals::empty();
    suffixes(&expr, &mut lits);
}

#[test]
fn test_suffixes_alternation_with_two_exprs() {
    let expr = Hir::alternation(vec![
        Hir::literal(Literal::Unicode('a')),
        Hir::literal(Literal::Unicode('b')),
    ]);
    let mut lits = Literals::empty();
    suffixes(&expr, &mut lits);
}

#[test]
fn test_suffixes_alternation_with_three_exprs() {
    let expr = Hir::alternation(vec![
        Hir::literal(Literal::Unicode('a')),
        Hir::literal(Literal::Unicode('b')),
        Hir::literal(Literal::Unicode('c')),
    ]);
    let mut lits = Literals::empty();
    suffixes(&expr, &mut lits);
}

#[test]
fn test_suffixes_alternation_with_five_exprs() {
    let expr = Hir::alternation(vec![
        Hir::literal(Literal::Unicode('a')),
        Hir::literal(Literal::Byte(0x61)),
        Hir::literal(Literal::Unicode('b')),
        Hir::literal(Literal::Byte(0x62)),
        Hir::literal(Literal::Unicode('c')),
    ]);
    let mut lits = Literals::empty();
    suffixes(&expr, &mut lits);
}

#[test]
fn test_suffixes_alternation_with_six_exprs() {
    let expr = Hir::alternation(vec![
        Hir::literal(Literal::Unicode('x')),
        Hir::literal(Literal::Unicode('y')),
        Hir::literal(Literal::Byte(0x7A)),
        Hir::literal(Literal::Unicode('z')),
        Hir::literal(Literal::Byte(0x79)),
        Hir::literal(Literal::Unicode('a')),
    ]);
    let mut lits = Literals::empty();
    suffixes(&expr, &mut lits);
}

#[test]
fn test_suffixes_alternation_with_nine_exprs() {
    let expr = Hir::alternation(vec![
        Hir::literal(Literal::Unicode('m')),
        Hir::literal(Literal::Unicode('n')),
        Hir::literal(Literal::Byte(0x6F)),
        Hir::literal(Literal::Byte(0x70)),
        Hir::literal(Literal::Unicode('q')),
        Hir::literal(Literal::Unicode('r')),
        Hir::literal(Literal::Byte(0x73)),
        Hir::literal(Literal::Unicode('t')),
        Hir::literal(Literal::Byte(0x75)),
    ]);
    let mut lits = Literals::empty();
    suffixes(&expr, &mut lits);
}

#[test]
fn test_suffixes_alternation_with_ten_exprs() {
    let expr = Hir::alternation(vec![
        Hir::literal(Literal::Unicode('i')),
        Hir::literal(Literal::Byte(0x6A)),
        Hir::literal(Literal::Unicode('j')),
        Hir::literal(Literal::Byte(0x6B)),
        Hir::literal(Literal::Unicode('k')),
        Hir::literal(Literal::Byte(0x6C)),
        Hir::literal(Literal::Unicode('l')),
        Hir::literal(Literal::Byte(0x6D)),
        Hir::literal(Literal::Unicode('m')),
        Hir::literal(Literal::Byte(0x6E)),
    ]);
    let mut lits = Literals::empty();
    suffixes(&expr, &mut lits);
}

