// Answer 0

#[test]
fn test_peek_valid_index() {
    let slice: &[u8] = &[1, 2, 3, 4, 5];
    let mut read = SliceRead { slice, index: 0 };
    let _result = read.peek();
}

#[test]
fn test_peek_middle_index() {
    let slice: &[u8] = &[10, 20, 30, 40, 50];
    let mut read = SliceRead { slice, index: 2 };
    let _result = read.peek();
}

#[test]
fn test_peek_last_index() {
    let slice: &[u8] = &[100, 200, 300];
    let mut read = SliceRead { slice, index: 2 };
    let _result = read.peek();
}

#[test]
fn test_peek_zero_index() {
    let slice: &[u8] = &[255];
    let mut read = SliceRead { slice, index: 0 };
    let _result = read.peek();
}

