// Answer 0

#[test]
fn test_visit_post_empty() {
    let mut buffer = Vec::new();
    let mut writer = std::fmt::Write::new(&mut buffer);
    let hir = Hir::new_empty();
    let mut printer = Printer { wtr: &mut writer };
    assert_eq!(printer.visit_post(&hir), Ok(()));
}

#[test]
fn test_visit_post_literal() {
    let mut buffer = Vec::new();
    let mut writer = std::fmt::Write::new(&mut buffer);
    let hir = Hir::new_literal(b'a');
    let mut printer = Printer { wtr: &mut writer };
    assert_eq!(printer.visit_post(&hir), Ok(()));
}

#[test]
fn test_visit_post_class() {
    let mut buffer = Vec::new();
    let mut writer = std::fmt::Write::new(&mut buffer);
    let hir = Hir::new_class(vec![b'a', b'b']);
    let mut printer = Printer { wtr: &mut writer };
    assert_eq!(printer.visit_post(&hir), Ok(()));
}

#[test]
fn test_visit_post_anchor() {
    let mut buffer = Vec::new();
    let mut writer = std::fmt::Write::new(&mut buffer);
    let hir = Hir::new_anchor(Anchor::Start);
    let mut printer = Printer { wtr: &mut writer };
    assert_eq!(printer.visit_post(&hir), Ok(()));
}

#[test]
fn test_visit_post_concat() {
    let mut buffer = Vec::new();
    let mut writer = std::fmt::Write::new(&mut buffer);
    let hir = Hir::new_concat(vec![Hir::new_literal(b'a'), Hir::new_literal(b'b')]);
    let mut printer = Printer { wtr: &mut writer };
    assert_eq!(printer.visit_post(&hir), Ok(()));
}

#[test]
fn test_visit_post_word_boundary() {
    let mut buffer = Vec::new();
    let mut writer = std::fmt::Write::new(&mut buffer);
    let hir = Hir::new_word_boundary(WordBoundary::Word);
    let mut printer = Printer { wtr: &mut writer };
    assert_eq!(printer.visit_post(&hir), Ok(()));
}

#[test]
fn test_visit_post_alternation() {
    let mut buffer = Vec::new();
    let mut writer = std::fmt::Write::new(&mut buffer);
    let hir = Hir::new_alternation(vec![Hir::new_literal(b'a'), Hir::new_literal(b'b')]);
    let mut printer = Printer { wtr: &mut writer };
    assert_eq!(printer.visit_post(&hir), Ok(()));
}

