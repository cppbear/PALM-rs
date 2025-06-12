// Answer 0

#[test]
fn test_start() {
    #[derive(Clone, Copy, Default, Eq, PartialEq, PartialOrd, Ord)]
    struct TestRange {
        start: char,
        end: char,
    }

    impl TestRange {
        pub fn new(start: char, end: char) -> TestRange {
            TestRange { start, end }
        }

        pub fn start(&self) -> char {
            self.start
        }
        
        pub fn end(&self) -> char {
            self.end
        }
    }

    let range = TestRange::new('a', 'z');
    assert_eq!(range.start(), 'a');
}

#[test]
fn test_start_equal_range() {
    #[derive(Clone, Copy, Default, Eq, PartialEq, PartialOrd, Ord)]
    struct TestRange {
        start: char,
        end: char,
    }

    impl TestRange {
        pub fn new(start: char, end: char) -> TestRange {
            TestRange { start, end }
        }

        pub fn start(&self) -> char {
            self.start
        }

        pub fn end(&self) -> char {
            self.end
        }
    }

    let range = TestRange::new('b', 'b');
    assert_eq!(range.start(), 'b');
}

#[test]
fn test_start_with_unicode() {
    #[derive(Clone, Copy, Default, Eq, PartialEq, PartialOrd, Ord)]
    struct TestRange {
        start: char,
        end: char,
    }

    impl TestRange {
        pub fn new(start: char, end: char) -> TestRange {
            TestRange { start, end }
        }

        pub fn start(&self) -> char {
            self.start
        }

        pub fn end(&self) -> char {
            self.end
        }
    }

    let range = TestRange::new('я', 'я');
    assert_eq!(range.start(), 'я');
}

