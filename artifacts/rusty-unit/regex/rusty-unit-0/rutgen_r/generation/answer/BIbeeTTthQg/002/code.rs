// Answer 0

#[derive(Debug)]
enum HirFrame {
    Expr(Hir),
    Other,
}

#[derive(Debug)]
struct Hir;

#[test]
fn test_unwrap_expr_valid() {
    let expr = Hir;
    let frame = HirFrame::Expr(expr);
    let result = unwrap_expr(frame);
    // Here we would typically assert something about `result` if needed
}

#[test]
#[should_panic(expected = "tried to unwrap expr from HirFrame, got: Other")]
fn test_unwrap_expr_panic() {
    let frame = HirFrame::Other;
    unwrap_expr(frame);
}

