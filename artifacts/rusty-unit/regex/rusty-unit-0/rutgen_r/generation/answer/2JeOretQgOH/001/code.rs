// Answer 0

#[test]
fn test_cls_char_count_empty() {
    struct ClassUnicode {
        ranges: Vec<Range>,
    }

    impl ClassUnicode {
        fn iter(&self) -> std::slice::Iter<Range> {
            self.ranges.iter()
        }
    }

    struct Range {
        start: usize,
        end: usize,
    }

    let cls = ClassUnicode { ranges: vec![] };
    assert_eq!(cls_char_count(&cls), 0);
}

#[test]
fn test_cls_char_count_single_range() {
    struct ClassUnicode {
        ranges: Vec<Range>,
    }

    impl ClassUnicode {
        fn iter(&self) -> std::slice::Iter<Range> {
            self.ranges.iter()
        }
    }

    struct Range {
        start: usize,
        end: usize,
    }

    let cls = ClassUnicode { ranges: vec![Range { start: 0, end: 5 }] };
    assert_eq!(cls_char_count(&cls), 6);
}

#[test]
fn test_cls_char_count_multiple_ranges() {
    struct ClassUnicode {
        ranges: Vec<Range>,
    }

    impl ClassUnicode {
        fn iter(&self) -> std::slice::Iter<Range> {
            self.ranges.iter()
        }
    }

    struct Range {
        start: usize,
        end: usize,
    }

    let cls = ClassUnicode { ranges: vec![Range { start: 0, end: 2 }, Range { start: 4, end: 6 }] };
    assert_eq!(cls_char_count(&cls), 5);
}

#[test]
fn test_cls_char_count_large_ranges() {
    struct ClassUnicode {
        ranges: Vec<Range>,
    }

    impl ClassUnicode {
        fn iter(&self) -> std::slice::Iter<Range> {
            self.ranges.iter()
        }
    }

    struct Range {
        start: usize,
        end: usize,
    }

    let cls = ClassUnicode { ranges: vec![Range { start: 0, end: 10_000 }, Range { start: 20_000, end: 30_000 }] };
    assert_eq!(cls_char_count(&cls), 20_001);
}

