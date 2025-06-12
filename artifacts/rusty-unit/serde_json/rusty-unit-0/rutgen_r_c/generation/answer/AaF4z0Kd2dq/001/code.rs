// Answer 0

#[test]
fn test_next_with_valid_index() {
    let mut slice_read = SliceRead {
        slice: &[10, 20, 30],
        index: 0,
    };
    let result = slice_read.next();
    assert_eq!(result, Ok(Some(10)));
    assert_eq!(slice_read.index, 1);
}

#[test]
fn test_next_with_valid_index_second_element() {
    let mut slice_read = SliceRead {
        slice: &[10, 20, 30],
        index: 1,
    };
    let result = slice_read.next();
    assert_eq!(result, Ok(Some(20)));
    assert_eq!(slice_read.index, 2);
}

#[test]
fn test_next_with_valid_index_third_element() {
    let mut slice_read = SliceRead {
        slice: &[10, 20, 30],
        index: 2,
    };
    let result = slice_read.next();
    assert_eq!(result, Ok(Some(30)));
    assert_eq!(slice_read.index, 3);
}

#[test]
fn test_next_at_boundary_condition() {
    let mut slice_read = SliceRead {
        slice: &[10, 20, 30],
        index: 3,
    };
    let result = slice_read.next();
    assert_eq!(result, Ok(None));
    assert_eq!(slice_read.index, 3);
}

