// Answer 0

#[test]
fn test_push_single_range() {
    let mut class_unicode = ClassUnicode::empty();
    let range = ClassUnicodeRange { start: 'a', end: 'z' };
    class_unicode.push(range);
    assert_eq!(class_unicode.ranges(), &[ClassUnicodeRange { start: 'a', end: 'z' }]);
}

#[test]
fn test_push_multiple_ranges() {
    let mut class_unicode = ClassUnicode::new(vec![
        ClassUnicodeRange { start: 'a', end: 'c' },
        ClassUnicodeRange { start: 'f', end: 'h' },
    ]);
    
    let new_range = ClassUnicodeRange { start: 'd', end: 'e' };
    class_unicode.push(new_range);
    
    assert_eq!(class_unicode.ranges(), &[
        ClassUnicodeRange { start: 'a', end: 'c' },
        ClassUnicodeRange { start: 'd', end: 'e' },
        ClassUnicodeRange { start: 'f', end: 'h' },
    ]);
}

#[test]
fn test_push_overlapping_ranges() {
    let mut class_unicode = ClassUnicode::new(vec![
        ClassUnicodeRange { start: 'a', end: 'c' },
    ]);
    
    let overlapping_range = ClassUnicodeRange { start: 'b', end: 'd' };
    class_unicode.push(overlapping_range);
    
    assert_eq!(class_unicode.ranges(), &[
        ClassUnicodeRange { start: 'a', end: 'c' },
        ClassUnicodeRange { start: 'b', end: 'd' },
    ]);
}

#[test]
fn test_push_range_same_start_end() {
    let mut class_unicode = ClassUnicode::empty();
    let single_char_range = ClassUnicodeRange { start: 'x', end: 'x' };
    class_unicode.push(single_char_range);
    
    assert_eq!(class_unicode.ranges(), &[ClassUnicodeRange { start: 'x', end: 'x' }]);
}

#[test]
fn test_push_empty_set() {
    let mut class_unicode = ClassUnicode::empty();
    let range = ClassUnicodeRange { start: '1', end: '9' };
    class_unicode.push(range);
    
    assert_eq!(class_unicode.ranges(), &[ClassUnicodeRange { start: '1', end: '9' }]);
}

