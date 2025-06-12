// Answer 0

#[derive(Debug)]
enum HirFrame {
    Expr(Hir),
    Other,
}

#[derive(Debug)]
struct Hir;

#[test]
#[should_panic(expected = "tried to unwrap expr from HirFrame, got: Other")]
fn test_unwrap_expr_should_panic_when_other_variant() {
    let frame = HirFrame::Other;
    frame.unwrap_expr();
}

#[test]
#[should_panic(expected = "tried to unwrap expr from HirFrame, got: Other")]
fn test_unwrap_expr_should_panic_for_other_frame() {
    let frame = HirFrame::Other;
    frame.unwrap_expr();
}

