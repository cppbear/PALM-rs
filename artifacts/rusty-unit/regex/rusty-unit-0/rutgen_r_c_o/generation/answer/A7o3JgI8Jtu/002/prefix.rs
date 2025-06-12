// Answer 0

#[test]
fn test_case_fold_simple_unicode_with_single_range() {
    let mut class = Class::Unicode(ClassUnicode::new(vec![ClassUnicodeRange::new(0x41, 0x41)])); // A
    class.case_fold_simple();
}

#[test]
fn test_case_fold_simple_unicode_with_multiple_ranges() {
    let mut class = Class::Unicode(ClassUnicode::new(vec![
        ClassUnicodeRange::new(0x41, 0x5A), // A-Z
        ClassUnicodeRange::new(0x61, 0x7A), // a-z
    ]));
    class.case_fold_simple();
}

#[test]
fn test_case_fold_simple_unicode_with_all_ascii() {
    let mut class = Class::Unicode(ClassUnicode::new(vec![ClassUnicodeRange::new(0x00, 0x7F)])); // all ASCII
    class.case_fold_simple();
}

#[test]
fn test_case_fold_simple_unicode_with_non_ascii() {
    let mut class = Class::Unicode(ClassUnicode::new(vec![
        ClassUnicodeRange::new(0x00C0, 0x00FF), // À-ÿ
    ]));
    class.case_fold_simple();
}

#[test]
fn test_case_fold_simple_unicode_with_large_range() {
    let mut class = Class::Unicode(ClassUnicode::new(vec![
        ClassUnicodeRange::new(0x0001, 0xFFFF),
    ]));
    class.case_fold_simple();
}

#[test]
fn test_case_fold_simple_unicode_with_empty_class() {
    let mut class = Class::Unicode(ClassUnicode::empty());
    class.case_fold_simple();
}

