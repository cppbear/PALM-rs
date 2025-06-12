// Answer 0

#[test]
fn test_skip_to_escape_slow_valid_escape() {
    struct TestStruct {
        index: usize,
        slice: Vec<char>,
    }

    impl TestStruct {
        fn skip_to_escape_slow(&mut self) {
            while self.index < self.slice.len() && !is_escape(self.slice[self.index], true) {
                self.index += 1;
            }
        }
    }

    fn is_escape(ch: char, _: bool) -> bool {
        ch == '\\'
    }

    let mut test_instance = TestStruct {
        index: 0,
        slice: vec!['a', 'b', 'c', '\\', 'd'],
    };
    test_instance.skip_to_escape_slow();
    assert_eq!(test_instance.index, 3);
}

#[test]
fn test_skip_to_escape_slow_no_escape() {
    struct TestStruct {
        index: usize,
        slice: Vec<char>,
    }

    impl TestStruct {
        fn skip_to_escape_slow(&mut self) {
            while self.index < self.slice.len() && !is_escape(self.slice[self.index], true) {
                self.index += 1;
            }
        }
    }

    fn is_escape(ch: char, _: bool) -> bool {
        ch == '\\'
    }

    let mut test_instance = TestStruct {
        index: 0,
        slice: vec!['a', 'b', 'c', 'd'],
    };
    test_instance.skip_to_escape_slow();
    assert_eq!(test_instance.index, 4); // should skip to the end
}

#[test]
fn test_skip_to_escape_slow_initial_index_at_end() {
    struct TestStruct {
        index: usize,
        slice: Vec<char>,
    }

    impl TestStruct {
        fn skip_to_escape_slow(&mut self) {
            while self.index < self.slice.len() && !is_escape(self.slice[self.index], true) {
                self.index += 1;
            }
        }
    }

    fn is_escape(ch: char, _: bool) -> bool {
        ch == '\\'
    }

    let mut test_instance = TestStruct {
        index: 4,
        slice: vec!['a', 'b', 'c', '\\'],
    };
    test_instance.skip_to_escape_slow();
    assert_eq!(test_instance.index, 4); // no change as the index is already at the end
}

