// Answer 0

#[test]
fn test_visit_pre_word_boundary_unicode_negate() {
    let mut output = String::new();
    let mut writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut output,
    };
    let hir = Hir {
        kind: HirKind::WordBoundary(hir::WordBoundary::UnicodeNegate),
        info: HirInfo::default(),
    };
    writer.visit_pre(&hir).unwrap();
}

#[test]
fn test_visit_pre_word_boundary_ascii() {
    let mut output = String::new();
    let mut writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut output,
    };
    let hir = Hir {
        kind: HirKind::WordBoundary(hir::WordBoundary::Ascii),
        info: HirInfo::default(),
    };
    writer.visit_pre(&hir).unwrap();
}

#[test]
fn test_visit_pre_word_boundary_ascii_negate() {
    let mut output = String::new();
    let mut writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut output,
    };
    let hir = Hir {
        kind: HirKind::WordBoundary(hir::WordBoundary::AsciiNegate),
        info: HirInfo::default(),
    };
    writer.visit_pre(&hir).unwrap();
}

#[test]
fn test_visit_pre_word_boundary_unicode() {
    let mut output = String::new();
    let mut writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut output,
    };
    let hir = Hir {
        kind: HirKind::WordBoundary(hir::WordBoundary::Unicode),
        info: HirInfo::default(),
    };
    writer.visit_pre(&hir).unwrap();
}

#[test]
fn test_visit_pre_anchor_start_line() {
    let mut output = String::new();
    let mut writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut output,
    };
    let hir = Hir {
        kind: HirKind::Anchor(hir::Anchor::StartLine),
        info: HirInfo::default(),
    };
    writer.visit_pre(&hir).unwrap();
}

#[test]
fn test_visit_pre_anchor_end_line() {
    let mut output = String::new();
    let mut writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut output,
    };
    let hir = Hir {
        kind: HirKind::Anchor(hir::Anchor::EndLine),
        info: HirInfo::default(),
    };
    writer.visit_pre(&hir).unwrap();
}

#[test]
fn test_visit_pre_anchor_start_text() {
    let mut output = String::new();
    let mut writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut output,
    };
    let hir = Hir {
        kind: HirKind::Anchor(hir::Anchor::StartText),
        info: HirInfo::default(),
    };
    writer.visit_pre(&hir).unwrap();
}

#[test]
fn test_visit_pre_anchor_end_text() {
    let mut output = String::new();
    let mut writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut output,
    };
    let hir = Hir {
        kind: HirKind::Anchor(hir::Anchor::EndText),
        info: HirInfo::default(),
    };
    writer.visit_pre(&hir).unwrap();
}

