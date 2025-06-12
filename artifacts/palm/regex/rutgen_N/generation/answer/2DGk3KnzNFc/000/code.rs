// Answer 0

#[derive(Debug)]
enum HirFrame {
    ClassUnicode(hir::ClassUnicode),
    Other,
}

mod hir {
    #[derive(Debug)]
    pub struct ClassUnicode;
}

#[test]
fn test_unwrap_class_unicode() {
    let cls = hir::ClassUnicode;
    let frame = HirFrame::ClassUnicode(cls);

    let unwrapped = frame.unwrap_class_unicode();
    assert_eq!(format!("{:?}", unwrapped), "ClassUnicode");
}

#[test]
#[should_panic(expected = "tried to unwrap Unicode class from HirFrame, got: Other")]
fn test_unwrap_class_unicode_panic() {
    let frame = HirFrame::Other;
    frame.unwrap_class_unicode();
}

