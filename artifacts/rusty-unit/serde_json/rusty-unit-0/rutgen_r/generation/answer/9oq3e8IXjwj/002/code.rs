// Answer 0

#[derive(Debug)]
struct SliceChecker {
    slice: Vec<u8>,
    index: usize,
}

impl SliceChecker {
    fn new(slice: Vec<u8>, index: usize) -> Self {
        Self { slice, index }
    }

    fn skip_to_escape_slow(&mut self) {
        while self.index < self.slice.len() && !is_escape(self.slice[self.index], true) {
            self.index += 1;
        }
    }
}

fn is_escape(byte: u8, _: bool) -> bool {
    byte == b'\\'
}

#[test]
fn test_skip_to_escape_slow_not_reached_end() {
    let mut checker = SliceChecker::new(vec![b'a', b'b', b'c', b'\\', b'd', b'e'], 0);
    checker.skip_to_escape_slow();
    assert_eq!(checker.index, 3);
}

#[test]
fn test_skip_to_escape_slow_reached_end_without_escape() {
    let mut checker = SliceChecker::new(vec![b'a', b'b', b'c', b'd', b'e'], 0);
    checker.skip_to_escape_slow();
    assert_eq!(checker.index, 5);
}

#[test]
fn test_skip_to_escape_slow_at_end_of_slice() {
    let mut checker = SliceChecker::new(vec![b'a', b'b', b'c'], 3);
    checker.skip_to_escape_slow();
    assert_eq!(checker.index, 3);
}

#[test]
#[should_panic]
fn test_skip_to_escape_slow_index_out_of_bounds() {
    let mut checker = SliceChecker::new(vec![b'a', b'b'], 3);
    checker.skip_to_escape_slow(); // This will not panic, but index will not be incremented
    assert_eq!(checker.index, 3);
}

