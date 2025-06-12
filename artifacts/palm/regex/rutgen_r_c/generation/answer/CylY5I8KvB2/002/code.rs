// Answer 0

#[test]
fn test_frame_child_concat() {
    use hir::{Hir, HirKind, Group, GroupKind, Repetition, RepetitionKind};

    // Initialize inner Hir structures
    let inner_hir_1 = Hir {
        kind: HirKind::Group(Group { 
            kind: GroupKind::Capturing(0), // Example group kind
            hir: Box::new(Hir { kind: HirKind::Empty, info: Default::default() }),
        }),
        info: Default::default(),
    };

    let inner_hir_2 = Hir {
        kind: HirKind::Repetition(Repetition { 
            kind: RepetitionKind::ZeroOrMore, // Example repetition kind
            greedy: true,
            hir: Box::new(Hir { kind: HirKind::Empty, info: Default::default() }),
        }),
        info: Default::default(),
    };

    // Create a concatenation Frame with multiple inner Hir nodes
    let head = &inner_hir_1; 
    let tail: &[Hir] = &[inner_hir_2];

    let frame = Frame::Concat { head, tail };

    // Call the child method and assert the result
    let result = frame.child();
    
    assert_eq!(result, head);
}

#[test]
fn test_frame_child_repetition() {
    use hir::{Hir, HirKind, Group, GroupKind, Repetition, RepetitionKind};

    // Create a Hir structure for a repetition
    let rep_hir = Hir {
        kind: HirKind::Repetition(Repetition { 
            kind: RepetitionKind::OneOrMore, // Example repetition kind
            greedy: false,
            hir: Box::new(Hir { kind: HirKind::Empty, info: Default::default() }),
        }),
        info: Default::default(),
    };

    // Create a repetition Frame
    let frame = Frame::Repetition(&Repetition {
        kind: RepetitionKind::OneOrMore,
        greedy: true,
        hir: Box::new(rep_hir),
    });

    // Call the child method and assert the result
    let result = frame.child();

    assert_eq!(result.kind, HirKind::Empty);
}

#[test]
fn test_frame_child_group() {
    use hir::{Hir, HirKind, Group, GroupKind};

    // Create a Hir structure for a group
    let group_hir = Hir {
        kind: HirKind::Group(Group { 
            kind: GroupKind::Capturing(1), // Example group kind
            hir: Box::new(Hir { kind: HirKind::Empty, info: Default::default() }),
        }),
        info: Default::default(),
    };

    // Create a group Frame
    let frame = Frame::Group(&Group {
        kind: GroupKind::Capturing(1),
        hir: Box::new(group_hir),
    });

    // Call the child method and assert the result
    let result = frame.child();

    assert_eq!(result.kind, HirKind::Empty);
}

#[test]
fn test_frame_child_alternation() {
    use hir::{Hir, HirKind};

    // Create a Hir structure for an alternation
    let alternation_hir = Hir {
        kind: HirKind::Alternation, // Example alternation kind
        info: Default::default(),
    };

    // Create an alternation Frame
    let frame = Frame::Alternation {
        head: &alternation_hir,
        tail: &[],
    };

    // Call the child method and assert the result
    let result = frame.child();

    assert_eq!(result.kind, HirKind::Alternation);
}

