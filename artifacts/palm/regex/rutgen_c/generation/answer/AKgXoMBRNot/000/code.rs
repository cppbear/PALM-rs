// Answer 0

#[test]
fn test_set_lower() {
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
    }

    let mut range = ClassUnicodeRange::default();
    range.set_lower('a');
    assert_eq!(range.lower(), 'a');
    
    range.set_lower('b');
    assert_eq!(range.lower(), 'b');
    
    range.set_lower('z');
    assert_eq!(range.lower(), 'z');
}

#[test]
fn test_set_lower_empty_range() {
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
    }

    let mut range = ClassUnicodeRange::default();
    range.set_lower(' ');
    assert_eq!(range.lower(), ' ');
    
    range.set_lower('!'); 
    assert_eq!(range.lower(), '!');
}

