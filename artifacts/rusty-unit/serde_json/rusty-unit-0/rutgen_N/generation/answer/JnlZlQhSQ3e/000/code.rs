// Answer 0

#[test]
fn test_position_of_index_first_line() {
    struct TestStruct {
        slice: Vec<u8>,
    }

    impl TestStruct {
        fn position_of_index(&self, i: usize) -> Position {
            let start_of_line = match memchr::memrchr(b'\n', &self.slice[..i]) {
                Some(position) => position + 1,
                None => 0,
            };
            Position {
                line: 1 + memchr::memchr_iter(b'\n', &self.slice[..start_of_line]).count(),
                column: i - start_of_line,
            }
        }
    }

    struct Position {
        line: usize,
        column: usize,
    }

    let data = b"Hello, World!\nThis is a test.";
    let test_struct = TestStruct {
        slice: data.to_vec(),
    };
    
    let pos = test_struct.position_of_index(5);
    assert_eq!(pos.line, 1);
    assert_eq!(pos.column, 5);
}

#[test]
fn test_position_of_index_second_line() {
    struct TestStruct {
        slice: Vec<u8>,
    }

    impl TestStruct {
        fn position_of_index(&self, i: usize) -> Position {
            let start_of_line = match memchr::memrchr(b'\n', &self.slice[..i]) {
                Some(position) => position + 1,
                None => 0,
            };
            Position {
                line: 1 + memchr::memchr_iter(b'\n', &self.slice[..start_of_line]).count(),
                column: i - start_of_line,
            }
        }
    }

    struct Position {
        line: usize,
        column: usize,
    }

    let data = b"Hello, World!\nThis is a test.";
    let test_struct = TestStruct {
        slice: data.to_vec(),
    };
    
    let pos = test_struct.position_of_index(15);
    assert_eq!(pos.line, 2);
    assert_eq!(pos.column, 0);
}

#[test]
fn test_position_of_index_no_newline() {
    struct TestStruct {
        slice: Vec<u8>,
    }

    impl TestStruct {
        fn position_of_index(&self, i: usize) -> Position {
            let start_of_line = match memchr::memrchr(b'\n', &self.slice[..i]) {
                Some(position) => position + 1,
                None => 0,
            };
            Position {
                line: 1 + memchr::memchr_iter(b'\n', &self.slice[..start_of_line]).count(),
                column: i - start_of_line,
            }
        }
    }

    struct Position {
        line: usize,
        column: usize,
    }

    let data = b"Hello, World!";
    let test_struct = TestStruct {
        slice: data.to_vec(),
    };
    
    let pos = test_struct.position_of_index(5);
    assert_eq!(pos.line, 1);
    assert_eq!(pos.column, 5);
}

