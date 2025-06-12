// Answer 0

#[test]
fn test_child_repetition() {
    let inner_hir = Hir { 
        kind: HirKind::SomeKind, // Replace `SomeKind` with actual variant
        info: HirInfo::new(), // Assuming HirInfo has a method `new`
    };
    let repetition = Frame::Repetition(Box::new(Repetition {
        kind: RepetitionKind::SomeKind, // Replace `SomeKind` with actual variant
        greedy: true,
        hir: Box::new(inner_hir.clone()),
    }));
    
    assert_eq!(repetition.child(), &inner_hir);
}

#[test]
fn test_child_group() {
    let inner_hir = Hir { 
        kind: HirKind::SomeKind, // Replace `SomeKind` with actual variant
        info: HirInfo::new(), // Assuming HirInfo has a method `new`
    };
    let group = Frame::Group(Box::new(Group {
        kind: GroupKind::SomeKind, // Replace `SomeKind` with actual variant
        hir: Box::new(inner_hir.clone()),
    }));

    assert_eq!(group.child(), &inner_hir);
}

#[test]
fn test_child_concat() {
    let head_hir = Hir { 
        kind: HirKind::SomeKind, // Replace `SomeKind` with actual variant
        info: HirInfo::new(), // Assuming HirInfo has a method `new`
    };
    let concat = Frame::Concat {
        head: &head_hir,
        tail: &[], // No remaining children
    };

    assert_eq!(concat.child(), &head_hir);
}

#[test]
fn test_child_alternation() {
    let head_hir = Hir { 
        kind: HirKind::SomeKind, // Replace `SomeKind` with actual variant
        info: HirInfo::new(), // Assuming HirInfo has a method `new`
    };
    let alternation = Frame::Alternation {
        head: &head_hir,
        tail: &[], // No remaining children
    };

    assert_eq!(alternation.child(), &head_hir);
}

