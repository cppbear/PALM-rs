// Answer 0

#[test]
fn test_peek_at_empty_slice() {
    let mut slice_read = SliceRead {
        slice: &[],
        index: 0,
    };
    let _ = slice_read.peek();
}

#[test]
fn test_peek_at_non_empty_slice_with_index_at_length() {
    let data = vec![1, 2, 3];
    let mut slice_read = SliceRead {
        slice: &data,
        index: 3,
    };
    let _ = slice_read.peek();
}

#[test]
fn test_peek_at_non_empty_slice_with_index_at_one() {
    let data = vec![1, 2, 3];
    let mut slice_read = SliceRead {
        slice: &data,
        index: 1,
    };
    let _ = slice_read.peek();
}

#[test]
fn test_peek_at_non_empty_slice_with_index_at_two() {
    let data = vec![4, 5, 6];
    let mut slice_read = SliceRead {
        slice: &data,
        index: 2,
    };
    let _ = slice_read.peek();
}

