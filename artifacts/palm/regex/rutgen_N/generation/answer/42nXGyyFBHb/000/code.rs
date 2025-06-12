// Answer 0

#[derive(Debug)]
struct Hir {
    kind: HirKind,
}

#[derive(Debug)]
enum HirKind {
    Empty,
    // other variants can be added as necessary
}

impl Hir {
    pub fn into_kind(mut self) -> HirKind {
        use std::mem;
        mem::replace(&mut self.kind, HirKind::Empty)
    }
}

#[test]
fn test_into_kind_empty() {
    let hir = Hir { kind: HirKind::Empty };
    assert_eq!(hir.into_kind(), HirKind::Empty);
}

#[test]
fn test_into_kind_non_empty() {
    let non_empty_hir = Hir { kind: HirKind::Empty }; // In a real situation, replace this with a variant that is not Empty
    assert_eq!(non_empty_hir.into_kind(), HirKind::Empty);
}

#[test]
fn test_into_kind_twice() {
    let mut hir = Hir { kind: HirKind::Empty };
    assert_eq!(hir.into_kind(), HirKind::Empty);
    assert_eq!(hir.into_kind(), HirKind::Empty); // Testing after first call
}

