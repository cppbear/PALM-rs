// Answer 0

#[test]
fn test_set_failed() {
    struct TestStruct<'a> {
        slice: &'a [i32],
        index: usize,
    }

    let mut failed = false;
    let mut test_instance = TestStruct {
        slice: &[1, 2, 3, 4, 5],
        index: 3,
    };

    test_instance.set_failed(&mut failed);
    assert_eq!(test_instance.slice, &[1, 2, 3]);
}

#[test]
fn test_set_failed_with_empty_slice() {
    struct TestStruct<'a> {
        slice: &'a [i32],
        index: usize,
    }

    let mut failed = false;
    let mut test_instance = TestStruct {
        slice: &[],
        index: 0,
    };

    test_instance.set_failed(&mut failed);
    assert_eq!(test_instance.slice, &[]);
}

#[test]
fn test_set_failed_with_index_equal_to_slice_length() {
    struct TestStruct<'a> {
        slice: &'a [i32],
        index: usize,
    }

    let mut failed = false;
    let mut test_instance = TestStruct {
        slice: &[1, 2, 3],
        index: 3,
    };

    test_instance.set_failed(&mut failed);
    assert_eq!(test_instance.slice, &[1, 2, 3]);
}

