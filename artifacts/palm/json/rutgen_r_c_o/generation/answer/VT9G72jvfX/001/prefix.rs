// Answer 0

#[test]
fn test_next_empty_slice() {
    let slice = &[];
    let mut read = SliceRead { slice, index: 0 };
    let result = read.next();
}

#[test]
fn test_next_single_element() {
    let slice = &[10];
    let mut read = SliceRead { slice, index: 0 };
    let result = read.next();
}

#[test]
fn test_next_multiple_elements() {
    let slice = &[100, 200, 255];
    let mut read = SliceRead { slice, index: 0 };
    let result_1 = read.next();
    let result_2 = read.next();
    let result_3 = read.next();
}

#[test]
fn test_next_exceeding_index() {
    let slice = &[1, 2, 3];
    let mut read = SliceRead { slice, index: 3 };
    let result = read.next();
}

#[test]
fn test_next_non_empty_slice() {
    let slice = &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let mut read = SliceRead { slice, index: 0 };
    for _ in 0..slice.len() {
        let result = read.next();
    }
}

