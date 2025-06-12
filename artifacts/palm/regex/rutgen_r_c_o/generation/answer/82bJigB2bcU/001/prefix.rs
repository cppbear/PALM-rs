// Answer 0

#[test]
fn test_add_char_class_with_empty_class() {
    let mut literals = Literals::empty();
    let class_unicode = hir::ClassUnicode { set: IntervalSet::new() };
    literals.add_char_class(&class_unicode);
}

#[test]
fn test_add_char_class_with_small_class() {
    let mut literals = Literals::empty();
    literals.set_limit_size(100);
    literals.set_limit_class(100);
    let class_unicode = hir::ClassUnicode { set: IntervalSet::from_ranges(vec![(97, 97)]) }; // 'a'
    literals.add_char_class(&class_unicode);
}

#[test]
fn test_add_char_class_with_border_limit_class() {
    let mut literals = Literals::empty();
    literals.set_limit_size(100);
    literals.set_limit_class(1000);
    let class_unicode = hir::ClassUnicode { set: IntervalSet::from_ranges(vec![(97, 122)]) }; // 'a' to 'z'
    literals.add_char_class(&class_unicode);
}

#[test]
fn test_add_char_class_exceed_class_limit() {
    let mut literals = Literals::empty();
    literals.set_limit_size(0);
    literals.set_limit_class(1);
    let class_unicode = hir::ClassUnicode { set: IntervalSet::from_ranges(vec![(97, 122)]) }; // 'a' to 'z'
    literals.add_char_class(&class_unicode);
}

#[test]
fn test_add_char_class_with_varied_classes() {
    let mut literals = Literals::empty();
    literals.set_limit_size(500);
    literals.set_limit_class(500);
    let class_unicode = hir::ClassUnicode { set: IntervalSet::from_ranges(vec![(32, 126)]) }; // Printable ASCII
    literals.add_char_class(&class_unicode);
}

#[test]
fn test_add_char_class_with_maximum_limit_size() {
    let mut literals = Literals::empty();
    literals.set_limit_size(1000);
    literals.set_limit_class(1000);
    let class_unicode = hir::ClassUnicode { set: IntervalSet::from_ranges(vec![(1000, 1000)]) }; // No characters
    literals.add_char_class(&class_unicode);
}

