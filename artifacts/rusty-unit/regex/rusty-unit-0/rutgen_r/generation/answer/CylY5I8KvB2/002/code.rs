// Answer 0

#[derive(Debug)]
struct Hir;

#[derive(Debug)]
enum Frame<'a> {
    Repetition { hir: &'a Hir },
    Group { hir: &'a Hir },
    Concat { head: &'a Hir },
    Alternation { head: &'a Hir },
}

#[test]
fn test_child_concat() {
    let hir = Hir;
    let frame = Frame::Concat { head: &hir };
    let result = frame.child();
    assert_eq!(result, &hir);
}

#[test]
fn test_child_repetition() {
    let hir = Hir;
    let frame = Frame::Repetition { hir: &hir };
    let result = frame.child();
    assert_eq!(result, &hir);
}

#[test]
fn test_child_group() {
    let hir = Hir;
    let frame = Frame::Group { hir: &hir };
    let result = frame.child();
    assert_eq!(result, &hir);
}

#[test]
fn test_child_alternation() {
    let hir = Hir;
    let frame = Frame::Alternation { head: &hir };
    let result = frame.child();
    assert_eq!(result, &hir);
}

