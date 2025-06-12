// Answer 0

#[derive(Debug)]
enum HirFrame {
    Expr(Hir),
    Other,
}

#[derive(Debug)]
struct Hir;

impl HirFrame {
    fn unwrap_expr(self) -> Hir {
        match self {
            HirFrame::Expr(expr) => expr,
            _ => panic!("tried to unwrap expr from HirFrame, got: {:?}", self),
        }
    }
}

#[test]
fn test_unwrap_expr_with_expr() {
    let expr = Hir;
    let frame = HirFrame::Expr(expr);
    let result = frame.unwrap_expr();
    // Assert that the result is of type Hir
    assert!(std::mem::discriminant(&result) == std::mem::discriminant(&Hir));
}

#[test]
#[should_panic(expected = "tried to unwrap expr from HirFrame, got:")]
fn test_unwrap_expr_with_other() {
    let frame = HirFrame::Other;
    let _ = frame.unwrap_expr(); // This should panic
}

