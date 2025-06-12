// Answer 0

#[derive(Debug)]
struct Repetition {
    hir: Hir,
}

#[derive(Debug)]
struct Group {
    hir: Hir,
}

#[derive(Debug)]
struct Hir;

#[derive(Debug)]
enum Frame<'a> {
    Repetition(Repetition),
    Group(Group),
    Concat { head: &'a Hir },
    Alternation { head: &'a Hir },
}

impl<'a> Frame<'a> {
    fn child(&self) -> &'a Hir {
        match *self {
            Frame::Repetition(ref rep) => &rep.hir,
            Frame::Group(ref group) => &group.hir,
            Frame::Concat { head, .. } => head,
            Frame::Alternation { head, .. } => head,
        }
    }
}

#[test]
fn test_child_repetition() {
    let hir_instance = Hir;
    let repetition_frame = Frame::Repetition(Repetition { hir: hir_instance });
    let result = repetition_frame.child();
    assert_eq!(std::ptr::addr_of!(*result), std::ptr::addr_of!(&hir_instance));
}

#[test]
fn test_child_group() {
    let hir_instance = Hir;
    let group_frame = Frame::Group(Group { hir: hir_instance });
    let result = group_frame.child();
    assert_eq!(std::ptr::addr_of!(*result), std::ptr::addr_of!(&hir_instance));
}

#[test]
fn test_child_concat() {
    let hir_instance = Hir;
    let concat_frame = Frame::Concat { head: &hir_instance };
    let result = concat_frame.child();
    assert_eq!(std::ptr::addr_of!(*result), std::ptr::addr_of!(&hir_instance));
}

#[test]
fn test_child_alternation() {
    let hir_instance = Hir;
    let alternation_frame = Frame::Alternation { head: &hir_instance };
    let result = alternation_frame.child();
    assert_eq!(std::ptr::addr_of!(*result), std::ptr::addr_of!(&hir_instance));
}

