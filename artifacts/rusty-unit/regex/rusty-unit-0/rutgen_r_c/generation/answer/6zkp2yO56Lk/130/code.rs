// Answer 0

#[test]
fn test_alternation_single_expression() {
    // Test with a single expression
    let literal_hir = Hir::literal(Literal::from_char('a')); // Assume Literal::from_char exists.
    let exprs = vec![literal_hir.clone()];
    let result = Hir::alternation(exprs);
    
    match result.kind() {
        HirKind::Literal(ref lit) => assert_eq!(*lit, literal_hir.into_kind()), // Check that the result matches the input literal
        _ => panic!("Expected a Literal kind"),
    }
}

#[test]
fn test_alternation_empty_expression() {
    // Test with an empty expression (this should return empty Hir)
    let exprs: Vec<Hir> = vec![];
    let result = Hir::alternation(exprs);
    
    match result.kind() {
        HirKind::Empty => {}, // Check that the result is of kind Empty
        _ => panic!("Expected an Empty kind"),
    }
}

#[test]
fn test_alternation_multiple_expressions() {
    // Test with multiple expressions
    let literal1 = Hir::literal(Literal::from_char('a')); // Assume Literal::from_char exists.
    let literal2 = Hir::literal(Literal::from_char('b'));
    let exprs = vec![literal1.clone(), literal2.clone()];
    let result = Hir::alternation(exprs.clone());

    match result.kind() {
        HirKind::Alternation(ref result_exprs) => {
            assert_eq!(result_exprs.len(), exprs.len());
            for (result_expr, expected_expr) in result_exprs.iter().zip(&exprs) {
                assert_eq!(*result_expr, *expected_expr); // Ensure that the output expressions match those provided
            }
        }
        _ => panic!("Expected an Alternation kind"),
    }
}

