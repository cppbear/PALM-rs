// Answer 0

#[derive(Debug)]
enum HirFrame {
    Group { old_flags: Option<Flags> },
    Other,
}

#[derive(Debug)]
struct Flags {
    // add relevant fields if needed
}

#[test]
fn test_unwrap_group_with_some_flags() {
    let frame = HirFrame::Group { old_flags: Some(Flags {}) };
    assert_eq!(frame.unwrap_group(), Some(Flags {}));
}

#[test]
#[should_panic(expected = "tried to unwrap group from HirFrame, got: Other")]
fn test_unwrap_group_with_no_flags() {
    let frame = HirFrame::Other;
    frame.unwrap_group();
}

#[test]
fn test_unwrap_group_with_none_flags() {
    let frame = HirFrame::Group { old_flags: None };
    assert_eq!(frame.unwrap_group(), None);
}

