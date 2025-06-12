// Answer 0

#[test]
fn test_empty_class_unicode() {
    let class_unicode = ClassUnicode::empty();
    assert_eq!(class_unicode.ranges(), &[]);
}

#[test]
fn test_empty_class_unicode_new() {
    let class_unicode = ClassUnicode::new(vec![]);
    assert_eq!(class_unicode.ranges(), &[]);
}

#[test]
fn test_empty_class_unicode_push() {
    let mut class_unicode = ClassUnicode::empty();
    class_unicode.push(ClassUnicodeRange { start: 'a', end: 'b' });
    assert_eq!(class_unicode.ranges(), &[ClassUnicodeRange { start: 'a', end: 'b' }]);
}

