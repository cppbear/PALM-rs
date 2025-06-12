// Answer 0

#[test]
fn test_intersect_with_empty_set() {
    let mut class_a = ClassUnicode::empty();
    let class_b = ClassUnicode::empty();
    class_a.intersect(&class_b);
    assert_eq!(class_a.ranges(), &[]);
}

#[test]
fn test_intersect_with_non_empty_set() {
    let range_a = ClassUnicodeRange { start: 'a', end: 'z' };
    let range_b = ClassUnicodeRange { start: 'm', end: 'q' };
    
    let mut class_a = ClassUnicode::new(vec![range_a]);
    let class_b = ClassUnicode::new(vec![range_b]);
    
    class_a.intersect(&class_b);
    
    let expected = vec![ClassUnicodeRange { start: 'm', end: 'q' }];
    assert_eq!(class_a.ranges(), &expected);
}

#[test]
fn test_intersect_non_overlapping_ranges() {
    let range_a = ClassUnicodeRange { start: 'a', end: 'z' };
    let range_b = ClassUnicodeRange { start: 'A', end: 'Z' };
    
    let mut class_a = ClassUnicode::new(vec![range_a]);
    let class_b = ClassUnicode::new(vec![range_b]);
    
    class_a.intersect(&class_b);
    
    assert_eq!(class_a.ranges(), &[]);
}

#[test]
fn test_intersect_with_identical_sets() {
    let range = ClassUnicodeRange { start: 'a', end: 'z' };
    
    let mut class_a = ClassUnicode::new(vec![range]);
    let class_b = ClassUnicode::new(vec![range]);
    
    class_a.intersect(&class_b);
    
    let expected = vec![range];
    assert_eq!(class_a.ranges(), &expected);
}

#[test]
fn test_intersect_with_multiple_ranges() {
    let range_a1 = ClassUnicodeRange { start: 'a', end: 'b' };
    let range_a2 = ClassUnicodeRange { start: 'd', end: 'f' };
    let range_b = ClassUnicodeRange { start: 'b', end: 'e' };
    
    let mut class_a = ClassUnicode::new(vec![range_a1, range_a2]);
    let class_b = ClassUnicode::new(vec![range_b]);
    
    class_a.intersect(&class_b);
    
    let expected = vec![ClassUnicodeRange { start: 'b', end: 'b' }, ClassUnicodeRange { start: 'd', end: 'e' }];
    assert_eq!(class_a.ranges(), &expected);
}

