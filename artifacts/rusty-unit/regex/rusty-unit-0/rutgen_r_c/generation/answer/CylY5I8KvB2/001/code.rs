// Answer 0

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Span;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Ast;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum GroupKind {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RepetitionKind {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RepetitionOp {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HirInfo;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Hir {
    kind: HirKind,
    info: HirInfo,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Group {
    pub kind: GroupKind,
    pub hir: Box<Hir>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Repetition {
    pub kind: RepetitionKind,
    pub greedy: bool,
    pub hir: Box<Hir>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum HirKind {}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Frame<'a> {
    Repetition(&'a Repetition),
    Group(&'a Group),
    Concat { head: &'a Hir, tail: &'a [Hir] },
    Alternation { head: &'a Hir, tail: &'a [Hir] },
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
fn test_child_alternation() {
    let inner_hir = Hir {
        kind: HirKind {},
        info: HirInfo {},
    };

    let alternation_frame = Frame::Alternation {
        head: &inner_hir,
        tail: &[],
    };

    let result = alternation_frame.child();
    assert_eq!(result, &inner_hir);
}

#[test]
fn test_child_concat() {
    let inner_hir1 = Hir {
        kind: HirKind {},
        info: HirInfo {},
    };

    let inner_hir2 = Hir {
        kind: HirKind {},
        info: HirInfo {},
    };

    let concat_frame = Frame::Concat {
        head: &inner_hir1,
        tail: &[inner_hir2.clone()],
    };

    let result = concat_frame.child();
    assert_eq!(result, &inner_hir1);
}

#[test]
fn test_child_group() {
    let inner_hir = Hir {
        kind: HirKind {},
        info: HirInfo {},
    };

    let group = Group {
        kind: GroupKind {},
        hir: Box::new(inner_hir.clone()),
    };

    let group_frame = Frame::Group(&group);
    let result = group_frame.child();
    assert_eq!(result, &inner_hir);
}

#[test]
fn test_child_repetition() {
    let inner_hir = Hir {
        kind: HirKind {},
        info: HirInfo {},
    };

    let repetition = Repetition {
        kind: RepetitionKind {},
        greedy: true,
        hir: Box::new(inner_hir.clone()),
    };

    let repetition_frame = Frame::Repetition(&repetition);
    let result = repetition_frame.child();
    assert_eq!(result, &inner_hir);
}

