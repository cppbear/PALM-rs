// Answer 0

#[test]
fn test_set_failed_with_empty_slice() {
    let mut failed = false;
    let mut reader = SliceRead {
        slice: &[],
        index: 0,
    };
    reader.set_failed(&mut failed);
}

#[test]
fn test_set_failed_with_slice_of_length_one() {
    let mut failed = false;
    let mut reader = SliceRead {
        slice: &[1],
        index: 1,
    };
    reader.set_failed(&mut failed);
}

#[test]
fn test_set_failed_with_slice_of_length_two() {
    let mut failed = false;
    let mut reader = SliceRead {
        slice: &[1, 2],
        index: 1,
    };
    reader.set_failed(&mut failed);
}

#[test]
fn test_set_failed_with_slice_of_length_three() {
    let mut failed = false;
    let mut reader = SliceRead {
        slice: &[1, 2, 3],
        index: 2,
    };
    reader.set_failed(&mut failed);
}

#[test]
fn test_set_failed_with_slice_equals_index() {
    let mut failed = false;
    let mut reader = SliceRead {
        slice: &[1, 2, 3],
        index: 3,
    };
    reader.set_failed(&mut failed);
}

#[test]
fn test_set_failed_at_zero_index() {
    let mut failed = false;
    let mut reader = SliceRead {
        slice: &[1, 2, 3],
        index: 0,
    };
    reader.set_failed(&mut failed);
}

