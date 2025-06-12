// Answer 0

#[test]
fn test_cls_char_count_empty() {
    struct ClassUnicode {
        ranges: Vec<std::ops::Range<u32>>,
    }

    impl ClassUnicode {
        fn iter(&self) -> std::slice::Iter<std::ops::Range<u32>> {
            self.ranges.iter()
        }
    }

    let cls = ClassUnicode { ranges: vec![] };
    assert_eq!(cls_char_count(&cls), 0);
}

#[test]
fn test_cls_char_count_single_range() {
    struct ClassUnicode {
        ranges: Vec<std::ops::Range<u32>>,
    }

    impl ClassUnicode {
        fn iter(&self) -> std::slice::Iter<std::ops::Range<u32>> {
            self.ranges.iter()
        }
    }

    let cls = ClassUnicode { ranges: vec![0..5] };
    assert_eq!(cls_char_count(&cls), 5);
}

#[test]
fn test_cls_char_count_multiple_ranges() {
    struct ClassUnicode {
        ranges: Vec<std::ops::Range<u32>>,
    }

    impl ClassUnicode {
        fn iter(&self) -> std::slice::Iter<std::ops::Range<u32>> {
            self.ranges.iter()
        }
    }

    let cls = ClassUnicode { ranges: vec![0..3, 5..8] };
    assert_eq!(cls_char_count(&cls), 6);
}

#[test]
fn test_cls_char_count_non_contiguous_ranges() {
    struct ClassUnicode {
        ranges: Vec<std::ops::Range<u32>>,
    }

    impl ClassUnicode {
        fn iter(&self) -> std::slice::Iter<std::ops::Range<u32>> {
            self.ranges.iter()
        }
    }

    let cls = ClassUnicode { ranges: vec![1..3, 5..6, 10..15] };
    assert_eq!(cls_char_count(&cls), 9);
}

