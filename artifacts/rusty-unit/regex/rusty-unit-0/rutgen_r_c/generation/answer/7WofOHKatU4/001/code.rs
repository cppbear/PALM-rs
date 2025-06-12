// Answer 0

#[test]
fn test_class_bytes_range_start() {
    struct ClassBytesRange {
        start: u8,
        end: u8,
    }

    impl ClassBytesRange {
        pub fn new(start: u8, end: u8) -> ClassBytesRange {
            ClassBytesRange { start, end }
        }

        pub fn start(&self) -> u8 {
            self.start
        }
    }

    let range = ClassBytesRange::new(5, 10);
    assert_eq!(range.start(), 5);

    let range_zero = ClassBytesRange::new(0, 255);
    assert_eq!(range_zero.start(), 0);

    let range_equal = ClassBytesRange::new(7, 7);
    assert_eq!(range_equal.start(), 7);

    let range_max = ClassBytesRange::new(255, 255);
    assert_eq!(range_max.start(), 255);
}

