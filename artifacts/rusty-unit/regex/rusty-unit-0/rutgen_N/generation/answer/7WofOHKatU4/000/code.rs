// Answer 0

#[test]
fn test_start_of_range() {
    struct Range {
        start: u8,
        end: u8,
    }

    impl Range {
        pub fn start(&self) -> u8 {
            self.start
        }
    }

    let range = Range { start: 5, end: 10 };
    assert_eq!(range.start(), 5);
}

#[test]
fn test_start_of_range_boundary() {
    struct Range {
        start: u8,
        end: u8,
    }

    impl Range {
        pub fn start(&self) -> u8 {
            self.start
        }
    }

    let range = Range { start: 0, end: 0 };
    assert_eq!(range.start(), 0);
}

