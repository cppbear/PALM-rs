// Answer 0

#[test]
fn test_case_fold_simple_no_mapping_1() {
    let range = ClassUnicodeRange::new('A', 'Z');
    let mut ranges = Vec::new();
    range.case_fold_simple(&mut ranges);
}

#[test]
fn test_case_fold_simple_no_mapping_2() {
    let range = ClassUnicodeRange::new('0', '9');
    let mut ranges = Vec::new();
    range.case_fold_simple(&mut ranges);
}

#[test]
fn test_case_fold_simple_no_mapping_3() {
    let range = ClassUnicodeRange::new('!', '@');
    let mut ranges = Vec::new();
    range.case_fold_simple(&mut ranges);
}

#[test]
fn test_case_fold_simple_no_mapping_4() {
    let range = ClassUnicodeRange::new('[' , '`');
    let mut ranges = Vec::new();
    range.case_fold_simple(&mut ranges);
}

#[test]
fn test_case_fold_simple_no_mapping_5() {
    let range = ClassUnicodeRange::new('{' , '~');
    let mut ranges = Vec::new();
    range.case_fold_simple(&mut ranges);
}

#[test]
fn test_case_fold_simple_no_mapping_6() {
    let range = ClassUnicodeRange::new('\u{1F600}', '\u{1F64F}'); // Emoji Range
    let mut ranges = Vec::new();
    range.case_fold_simple(&mut ranges);
}

