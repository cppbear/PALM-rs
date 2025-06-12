// Answer 0

#[test]
fn test_upper() {
    struct UnicodeRange {
        start: char,
        end: char,
    }

    impl Interval for UnicodeRange {
        type Bound = char;

        fn lower(&self) -> char {
            self.start
        }

        fn upper(&self) -> char {
            self.end
        }

        fn set_lower(&mut self, bound: char) {
            self.start = bound;
        }

        fn set_upper(&mut self, bound: char) {
            self.end = bound;
        }

        fn case_fold_simple(&self, _: &mut Vec<Self>) {}
        
        fn is_contiguous(&self, other: &Self) -> bool {
            self.upper() >= other.lower() && self.lower() <= other.upper()
        }

        fn is_intersection_empty(&self, other: &Self) -> bool {
            self.upper() < other.lower() || self.lower() > other.upper()
        }

        fn is_subset(&self, other: &Self) -> bool {
            self.lower() >= other.lower() && self.upper() <= other.upper()
        }
    }

    let range = UnicodeRange { start: 'a', end: 'z' };
    assert_eq!(range.upper(), 'z');
}

#[test]
fn test_upper_empty_range() {
    struct EmptyRange {
        start: char,
        end: char,
    }

    impl Interval for EmptyRange {
        type Bound = char;

        fn lower(&self) -> char {
            self.start
        }

        fn upper(&self) -> char {
            self.end
        }

        fn set_lower(&mut self, bound: char) {
            self.start = bound;
        }

        fn set_upper(&mut self, bound: char) {
            self.end = bound;
        }

        fn case_fold_simple(&self, _: &mut Vec<Self>) {}
        
        fn is_contiguous(&self, other: &Self) -> bool {
            self.upper() >= other.lower() && self.lower() <= other.upper()
        }

        fn is_intersection_empty(&self, other: &Self) -> bool {
            self.upper() < other.lower() || self.lower() > other.upper()
        }

        fn is_subset(&self, other: &Self) -> bool {
            self.lower() >= other.lower() && self.upper() <= other.upper()
        }
    }

    let range = EmptyRange { start: '\0', end: '\0' };
    assert_eq!(range.upper(), '\0');
}

