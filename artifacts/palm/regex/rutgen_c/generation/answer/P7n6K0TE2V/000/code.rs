// Answer 0

#[test]
fn test_lower_method() {
    #[derive(Clone, Copy, Default, Eq, PartialEq, PartialOrd, Ord)]
    struct ClassUnicodeRange {
        start: char,
        end: char,
    }

    impl Interval for ClassUnicodeRange {
        type Bound = char;
        
        #[inline]
        fn lower(&self) -> char {
            self.start
        }

        #[inline]
        fn upper(&self) -> char {
            self.end
        }

        #[inline]
        fn set_lower(&mut self, bound: char) {
            self.start = bound;
        }

        #[inline]
        fn set_upper(&mut self, bound: char) {
            self.end = bound;
        }

        fn case_fold_simple(&self, ranges: &mut Vec<ClassUnicodeRange>) {}
        
        fn is_contiguous(&self, other: &Self) -> bool { true }
        fn is_intersection_empty(&self, other: &Self) -> bool { false }
        fn is_subset(&self, other: &Self) -> bool { false }
    }

    let range = ClassUnicodeRange { start: 'a', end: 'z' };
    assert_eq!(range.lower(), 'a');
}

#[test]
fn test_lower_method_with_different_chars() {
    let range = ClassUnicodeRange { start: 'A', end: 'Z' };
    assert_eq!(range.lower(), 'A');
}

