// Answer 0

#[test]
fn test_cls_char_count_empty() {
    struct ClassUnicodeRange {
        start: char,
        end: char,
    }

    let class_unicode = ClassUnicode::empty();
    assert_eq!(cls_char_count(&class_unicode), 0);
}

#[test]
fn test_cls_char_count_single_range() {
    struct ClassUnicodeRange {
        start: char,
        end: char,
    }

    let range = ClassUnicodeRange { start: 'a', end: 'a' };
    let class_unicode = ClassUnicode::new(vec![range]);
    assert_eq!(cls_char_count(&class_unicode), 1);
}

#[test]
fn test_cls_char_count_multiple_ranges() {
    struct ClassUnicodeRange {
        start: char,
        end: char,
    }

    let ranges = vec![
        ClassUnicodeRange { start: 'a', end: 'c' }, 
        ClassUnicodeRange { start: 'e', end: 'f' }
    ];
    let class_unicode = ClassUnicode::new(ranges);
    assert_eq!(cls_char_count(&class_unicode), 5);
}

#[test]
fn test_cls_char_count_overlapping_ranges() {
    struct ClassUnicodeRange {
        start: char,
        end: char,
    }

    let ranges = vec![
        ClassUnicodeRange { start: 'a', end: 'd' }, 
        ClassUnicodeRange { start: 'c', end: 'f' }
    ];
    let class_unicode = ClassUnicode::new(ranges);
    assert_eq!(cls_char_count(&class_unicode), 6);
}

#[test]
fn test_cls_char_count_ranges_with_gaps() {
    struct ClassUnicodeRange {
        start: char,
        end: char,
    }

    let ranges = vec![
        ClassUnicodeRange { start: 'a', end: 'b' },
        ClassUnicodeRange { start: 'd', end: 'e' }
    ];
    let class_unicode = ClassUnicode::new(ranges);
    assert_eq!(cls_char_count(&class_unicode), 4);
}

