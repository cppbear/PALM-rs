// Answer 0

#[test]
fn test_upper_with_valid_char() {
    struct TestStruct {
        end: char,
    }

    impl TestStruct {
        fn upper(&self) -> char {
            self.end
        }
    }

    let test_instance = TestStruct { end: 'A' };
    assert_eq!(test_instance.upper(), 'A');
}

#[test]
fn test_upper_with_different_valid_char() {
    struct TestStruct {
        end: char,
    }

    impl TestStruct {
        fn upper(&self) -> char {
            self.end
        }
    }

    let test_instance = TestStruct { end: 'Z' };
    assert_eq!(test_instance.upper(), 'Z');
}

#[test]
fn test_upper_with_non_alphabet_char() {
    struct TestStruct {
        end: char,
    }

    impl TestStruct {
        fn upper(&self) -> char {
            self.end
        }
    }

    let test_instance = TestStruct { end: '1' };
    assert_eq!(test_instance.upper(), '1');
}

#[test]
fn test_upper_with_special_char() {
    struct TestStruct {
        end: char,
    }

    impl TestStruct {
        fn upper(&self) -> char {
            self.end
        }
    }

    let test_instance = TestStruct { end: '!' };
    assert_eq!(test_instance.upper(), '!');
}

