// Answer 0

#[test]
fn test_iter_non_empty_class_unicode() {
    let range1 = ClassUnicodeRange { start: 'a', end: 'z' };
    let range2 = ClassUnicodeRange { start: 'A', end: 'Z' };
    let interval_set = IntervalSet::new(vec![range1, range2]);
    let class_unicode = ClassUnicode { set: interval_set };

    let mut iter = class_unicode.iter();
    let ranges_collected: Vec<ClassUnicodeRange> = iter.0.0.collect();
    
    assert_eq!(ranges_collected.len(), 2);
    assert_eq!(ranges_collected[0], ClassUnicodeRange { start: 'a', end: 'z' });
    assert_eq!(ranges_collected[1], ClassUnicodeRange { start: 'A', end: 'Z' });
}

#[test]
fn test_iter_empty_class_unicode() {
    let class_unicode = ClassUnicode::empty();
    
    let mut iter = class_unicode.iter();
    let ranges_collected: Vec<ClassUnicodeRange> = iter.0.0.collect();
    
    assert_eq!(ranges_collected.len(), 0);
}

