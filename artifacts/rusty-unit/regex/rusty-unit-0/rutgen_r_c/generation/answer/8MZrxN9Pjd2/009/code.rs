// Answer 0

#[test]
fn test_visit_pre_empty() {
    let mut output = Vec::new();
    let mut writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut output,
    };
    let hir = Hir::empty();
    let result = writer.visit_pre(&hir);
    assert_eq!(result, Ok(()));
    assert_eq!(String::from_utf8(output).unwrap(), "");
}

#[test]
fn test_visit_pre_repetition() {
    let mut output = Vec::new();
    let mut writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut output,
    };
    let hir = Hir::repetition(hir::Repetition::new());
    let result = writer.visit_pre(&hir);
    assert_eq!(result, Ok(()));
    assert_eq!(String::from_utf8(output).unwrap(), "");
}

#[test]
fn test_visit_pre_concat() {
    let mut output = Vec::new();
    let mut writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut output,
    };
    let hir = Hir::concat(vec![Hir::empty(), Hir::empty()]);
    let result = writer.visit_pre(&hir);
    assert_eq!(result, Ok(()));
    assert_eq!(String::from_utf8(output).unwrap(), "");
}

#[test]
fn test_visit_pre_alternation() {
    let mut output = Vec::new();
    let mut writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut output,
    };
    let hir = Hir::alternation(vec![Hir::empty(), Hir::empty()]);
    let result = writer.visit_pre(&hir);
    assert_eq!(result, Ok(()));
    assert_eq!(String::from_utf8(output).unwrap(), "");
}

