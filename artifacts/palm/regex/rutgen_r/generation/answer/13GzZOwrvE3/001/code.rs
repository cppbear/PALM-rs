// Answer 0

#[test]
fn test_cls_byte_count_empty() {
    struct ClassBytes {
        ranges: Vec<(u8, u8)>,
    }

    impl ClassBytes {
        fn iter(&self) -> impl Iterator<Item = (u8, u8)> + '_ {
            self.ranges.iter().copied()
        }
    }

    let cls = ClassBytes { ranges: Vec::new() };
    assert_eq!(cls_byte_count(&cls), 0);
}

#[test]
fn test_cls_byte_count_single_range() {
    struct ClassBytes {
        ranges: Vec<(u8, u8)>,
    }

    impl ClassBytes {
        fn iter(&self) -> impl Iterator<Item = (u8, u8)> + '_ {
            self.ranges.iter().copied()
        }
    }

    let cls = ClassBytes { ranges: vec![(5, 10)] };
    assert_eq!(cls_byte_count(&cls), 6); // 1 + (10 - 5) = 6
}

#[test]
fn test_cls_byte_count_multiple_ranges() {
    struct ClassBytes {
        ranges: Vec<(u8, u8)>,
    }

    impl ClassBytes {
        fn iter(&self) -> impl Iterator<Item = (u8, u8)> + '_ {
            self.ranges.iter().copied()
        }
    }

    let cls = ClassBytes { ranges: vec![(1, 3), (5, 8)] };
    assert_eq!(cls_byte_count(&cls), 7); // (1 + (3-1)) + (1 + (8-5)) = 3 + 4 = 7
}

#[test]
fn test_cls_byte_count_overlapping_ranges() {
    struct ClassBytes {
        ranges: Vec<(u8, u8)>,
    }

    impl ClassBytes {
        fn iter(&self) -> impl Iterator<Item = (u8, u8)> + '_ {
            self.ranges.iter().copied()
        }
    }

    let cls = ClassBytes { ranges: vec![(1, 5), (4, 6)] };
    assert_eq!(cls_byte_count(&cls), 6); // (1 + (5-1)) + (1 + (6-4)) = 5 + 3 = 6
}

#[test]
fn test_cls_byte_count_edge_case() {
    struct ClassBytes {
        ranges: Vec<(u8, u8)>,
    }

    impl ClassBytes {
        fn iter(&self) -> impl Iterator<Item = (u8, u8)> + '_ {
            self.ranges.iter().copied()
        }
    }

    let cls = ClassBytes { ranges: vec![(u8::MAX - 1, u8::MAX)] };
    assert_eq!(cls_byte_count(&cls), 2); // (1 + (MAX - (MAX - 1))) = 2
}

