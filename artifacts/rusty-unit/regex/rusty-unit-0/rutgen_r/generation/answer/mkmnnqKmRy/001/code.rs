// Answer 0

#[derive(Default)]
struct Parser {
    pos: Position,
}

#[derive(Default)]
struct Position {
    offset: usize,
}

struct TestStruct {
    parser: Parser,
}

impl TestStruct {
    fn parser(&self) -> &Parser {
        &self.parser
    }

    fn offset(&self) -> usize {
        self.parser().pos.offset
    }
}

#[test]
fn test_offset_zero() {
    let test_struct = TestStruct::default(); // initializes with default values
    assert_eq!(test_struct.offset(), 0); // checks offset at default (zero)
}

#[test]
fn test_offset_non_zero() {
    let mut test_struct = TestStruct::default();
    test_struct.parser.pos.offset = 5; // directly setting offset to a non-zero value
    assert_eq!(test_struct.offset(), 5); // checks expected non-zero offset
}

#[test]
fn test_offset_large_value() {
    let mut test_struct = TestStruct::default();
    test_struct.parser.pos.offset = usize::MAX; // sets offset to the maximum possible value
    assert_eq!(test_struct.offset(), usize::MAX); // checks the maximum value case
}

#[test]
fn test_offset_after_increment() {
    let mut test_struct = TestStruct::default();
    test_struct.parser.pos.offset = 10; // set initial offset
    assert_eq!(test_struct.offset(), 10); // asserting initial value
    test_struct.parser.pos.offset += 1; // simulate increment
    assert_eq!(test_struct.offset(), 11); // asserting new value after increment
}

