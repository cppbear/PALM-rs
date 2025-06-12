// Answer 0

#[test]
fn test_anchor_start_text() {
    let anchor = Anchor::StartText;
    let result = Hir::anchor(anchor);
}

#[test]
fn test_anchor_end_text() {
    let anchor = Anchor::EndText;
    let result = Hir::anchor(anchor);
}

