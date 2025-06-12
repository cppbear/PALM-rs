// Answer 0

#[test]
fn test_visit_pre_anchor_start_line() {
    let mut output = String::new();
    let mut writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut output,
    };
    let hir = Hir::anchor(hir::Anchor::StartLine);
    writer.visit_pre(&hir).unwrap();
}

#[test]
fn test_visit_pre_anchor_end_line() {
    let mut output = String::new();
    let mut writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut output,
    };
    let hir = Hir::anchor(hir::Anchor::EndLine);
    writer.visit_pre(&hir).unwrap();
}

#[test]
fn test_visit_pre_anchor_start_text() {
    let mut output = String::new();
    let mut writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut output,
    };
    let hir = Hir::anchor(hir::Anchor::StartText);
    writer.visit_pre(&hir).unwrap();
}

#[test]
fn test_visit_pre_anchor_end_text() {
    let mut output = String::new();
    let mut writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut output,
    };
    let hir = Hir::anchor(hir::Anchor::EndText);
    writer.visit_pre(&hir).unwrap();
}

#[test]
fn test_visit_pre_class_unicode() {
    let mut output = String::new();
    let mut writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut output,
    };
    let unicode_range = ClassUnicodeRange::new('\u{0030}', '\u{0039}');
    let class_unicode = ClassUnicode::new(vec![unicode_range]);
    let hir = Hir::class(hir::Class::Unicode(class_unicode));
    writer.visit_pre(&hir).unwrap();
}

#[test]
fn test_visit_pre_class_bytes() {
    let mut output = String::new();
    let mut writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut output,
    };
    let bytes_range = ClassBytesRange::new(0, 255);
    let class_bytes = ClassBytes::new(vec![bytes_range]);
    let hir = Hir::class(hir::Class::Bytes(class_bytes));
    writer.visit_pre(&hir).unwrap();
}

