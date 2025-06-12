// Answer 0

#[derive(Debug, PartialEq)]
struct Hir {
    kind: HirKind,
}

#[derive(Debug, PartialEq)]
enum HirKind {
    Literal(char),
    Concatenation(Vec<Box<Hir>>),
}

impl Hir {
    pub fn kind(&self) -> &HirKind {
        &self.kind
    }
}

#[test]
fn test_kind_literal() {
    let hir = Hir {
        kind: HirKind::Literal('a'),
    };
    assert_eq!(hir.kind(), &HirKind::Literal('a'));
}

#[test]
fn test_kind_concatenation() {
    let hir1 = Hir {
        kind: HirKind::Literal('b'),
    };
    let hir2 = Hir {
        kind: HirKind::Literal('c'),
    };
    let hir = Hir {
        kind: HirKind::Concatenation(vec![Box::new(hir1), Box::new(hir2)]),
    };
    assert_eq!(hir.kind(), &HirKind::Concatenation(vec![
        Box::new(Hir { kind: HirKind::Literal('b') }),
        Box::new(Hir { kind: HirKind::Literal('c') }),
    ]));
}

#[test]
fn test_kind_empty_concatenation() {
    let hir = Hir {
        kind: HirKind::Concatenation(vec![]),
    };
    assert_eq!(hir.kind(), &HirKind::Concatenation(vec![]));
}

