// Answer 0

#[derive(Debug)]
enum HirFrame {
    Group { old_flags: Option<Flags> },
    Other,
}

#[derive(Debug)]
struct Flags;

#[test]
#[should_panic(expected = "tried to unwrap group from HirFrame")]
fn test_unwrap_group_non_group() {
    let frame = HirFrame::Other;
    frame.unwrap_group();
}

#[test]
fn test_unwrap_group_none() {
    let frame = HirFrame::Group { old_flags: None };
    assert_eq!(frame.unwrap_group(), None);
}

#[test]
fn test_unwrap_group_some() {
    let frame = HirFrame::Group { old_flags: Some(Flags) };
    assert!(frame.unwrap_group().is_some());
}

