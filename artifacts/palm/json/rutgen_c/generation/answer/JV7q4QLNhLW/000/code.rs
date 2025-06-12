// Answer 0

#[test]
fn test_set_failed_empty_slice() {
    let mut slice: &[u8] = &[];
    let index = 0;
    let mut read = SliceRead { slice, index };

    let mut failed = false;
    read.set_failed(&mut failed);

    assert_eq!(read.slice.len(), 0);
}

#[test]
fn test_set_failed_non_empty_slice() {
    let data: &[u8] = &[1, 2, 3, 4, 5];
    let index = 3;
    let mut read = SliceRead {
        slice: data,
        index,
    };

    let mut failed = false;
    read.set_failed(&mut failed);

    assert_eq!(read.slice, &[1, 2, 3]);
}

#[test]
fn test_set_failed_with_index_zero() {
    let data: &[u8] = &[10, 20, 30];
    let index = 0;
    let mut read = SliceRead {
        slice: data,
        index,
    };

    let mut failed = false;
    read.set_failed(&mut failed);

    assert_eq!(read.slice.len(), 0);
}

#[test]
fn test_set_failed_with_same_index() {
    let data: &[u8] = &[100, 200, 300];
    let index = 3;
    let mut read = SliceRead {
        slice: data,
        index,
    };

    let mut failed = true;
    read.set_failed(&mut failed);

    assert_eq!(read.slice, &[100, 200, 300]);
}

