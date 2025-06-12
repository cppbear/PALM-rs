// Answer 0

#[test]
fn test_case_fold_simple_valid_range() {
    let mut ranges = Vec::new();
    let range = ClassUnicodeRange::new('\u{0041}', '\u{0042}');
    range.case_fold_simple(&mut ranges);
}

#[test]
fn test_case_fold_simple_empty_range() {
    let mut ranges = Vec::new();
    let range = ClassUnicodeRange::new('\u{007F}', '\u{007F}');
    range.case_fold_simple(&mut ranges);
}

#[test]
fn test_case_fold_simple_single_char_range() {
    let mut ranges = Vec::new();
    let range = ClassUnicodeRange::new('\u{0061}', '\u{0061}');
    range.case_fold_simple(&mut ranges);
}

#[test]
fn test_case_fold_simple_no_mapping() {
    let mut ranges = Vec::new();
    let range = ClassUnicodeRange::new('\u{0041}', '\u{0041}');
    range.case_fold_simple(&mut ranges);
}

#[test]
fn test_case_fold_simple_invalid_range() {
    let mut ranges = Vec::new();
    let range = ClassUnicodeRange::new('\u{0042}', '\u{0041}');
    range.case_fold_simple(&mut ranges);
}

