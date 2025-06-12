// Answer 0

#[test]
fn test_class_unicode_intersect_empty() {
    let mut class_a = ClassUnicode::empty();
    let class_b = ClassUnicode::empty();
    class_a.intersect(&class_b);
    assert_eq!(class_a.ranges(), &[]);
}

#[test]
fn test_class_unicode_intersect_non_empty() {
    let range_a = ClassUnicodeRange { start: 'a', end: 'z' };
    let range_b = ClassUnicodeRange { start: 'm', end: 'r' };
    
    let mut class_a = ClassUnicode::new(vec![range_a]);
    let class_b = ClassUnicode::new(vec![range_b]);
    
    class_a.intersect(&class_b);
    assert_eq!(class_a.ranges(), &[ClassUnicodeRange { start: 'm', end: 'r' }]);
}

#[test]
fn test_class_unicode_intersect_no_overlap() {
    let range_a = ClassUnicodeRange { start: 'a', end: 'm' };
    let range_b = ClassUnicodeRange { start: 'n', end: 'z' };
    
    let mut class_a = ClassUnicode::new(vec![range_a]);
    let class_b = ClassUnicode::new(vec![range_b]);
    
    class_a.intersect(&class_b);
    assert_eq!(class_a.ranges(), &[]);
}

#[test]
fn test_class_unicode_intersect_partial_overlap() {
    let range_a = ClassUnicodeRange { start: 'a', end: 'z' };
    let range_b = ClassUnicodeRange { start: 'x', end: 'y' };
    
    let mut class_a = ClassUnicode::new(vec![range_a]);
    let class_b = ClassUnicode::new(vec![range_b]);
    
    class_a.intersect(&class_b);
    assert_eq!(class_a.ranges(), &[ClassUnicodeRange { start: 'x', end: 'y' }]);
}

