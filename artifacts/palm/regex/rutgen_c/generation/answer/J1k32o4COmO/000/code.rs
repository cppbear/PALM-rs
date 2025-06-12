// Answer 0

#[test]
fn test_unwrap_group_with_none() {
    struct DummyHirFrame;

    let frame = HirFrame::Group { old_flags: None };
    assert_eq!(frame.unwrap_group(), None);
}

#[test]
#[should_panic(expected = "tried to unwrap group from HirFrame")]
fn test_unwrap_group_with_invalid_variant() {
    let frame = HirFrame::Expr(Hir { kind: HirKind::SomeKind, info: HirInfo::default() });
    frame.unwrap_group();
}

#[test]
fn test_unwrap_group_with_some_flags() {
    struct DummyHirFrame;

    let flags = Flags { case_insensitive: Some(true), multi_line: None, dot_matches_new_line: None, swap_greed: None, unicode: None };
    let frame = HirFrame::Group { old_flags: Some(flags) };
    assert_eq!(frame.unwrap_group(), Some(flags));
}

