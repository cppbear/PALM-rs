// Answer 0

#[test]
fn test_case_fold_simple_with_lowercase() {
    let mut class_unicode = ClassUnicode::new(vec![
        ClassUnicodeRange { start: 'a', end: 'z' },
    ]);
    class_unicode.case_fold_simple();
}

#[test]
fn test_case_fold_simple_with_uppercase() {
    let mut class_unicode = ClassUnicode::new(vec![
        ClassUnicodeRange { start: 'A', end: 'Z' },
    ]);
    class_unicode.case_fold_simple();
}

#[test]
fn test_case_fold_simple_with_alphanumeric() {
    let mut class_unicode = ClassUnicode::new(vec![
        ClassUnicodeRange { start: 'a', end: 'z' },
        ClassUnicodeRange { start: 'A', end: 'Z' },
        ClassUnicodeRange { start: '0', end: '9' },
    ]);
    class_unicode.case_fold_simple();
}

#[test]
fn test_case_fold_simple_with_empty_class() {
    let mut class_unicode = ClassUnicode::empty();
    class_unicode.case_fold_simple();
}

#[test]
fn test_case_fold_simple_with_mixed_ranges() {
    let mut class_unicode = ClassUnicode::new(vec![
        ClassUnicodeRange { start: 'a', end: 'z' },
        ClassUnicodeRange { start: '0', end: '9' },
    ]);
    class_unicode.case_fold_simple();
}

#[test]
fn test_case_fold_simple_with_non_letter_ranges() {
    let mut class_unicode = ClassUnicode::new(vec![
        ClassUnicodeRange { start: '!', end: '/' },
        ClassUnicodeRange { start: ':', end: '@' },
    ]);
    class_unicode.case_fold_simple();
}

