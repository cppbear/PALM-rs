// Answer 0

#[test]
fn test_erase_indices_no_erasure() {
    struct TestStruct {
        entries: Vec<u32>,
        indices: Vec<usize>,
    }

    let mut test_instance = TestStruct {
        entries: vec![1, 2, 3, 4, 5],
        indices: vec![0, 1, 2, 3, 4],
    };

    test_instance.erase_indices(2, 2);
    assert_eq!(test_instance.indices, vec![0, 1, 2, 3, 4]);
}

#[test]
fn test_erase_indices_with_erasure_and_shift_left() {
    struct TestStruct {
        entries: Vec<u32>,
        indices: Vec<usize>,
    }

    let mut test_instance = TestStruct {
        entries: vec![1, 2, 3, 4, 5],
        indices: vec![0, 1, 2, 3, 4],
    };

    test_instance.erase_indices(1, 3);
    assert_eq!(test_instance.indices, vec![0, 1, 3]);
}

#[test]
fn test_erase_indices_with_full_erase() {
    struct TestStruct {
        entries: Vec<u32>,
        indices: Vec<usize>,
    }

    let mut test_instance = TestStruct {
        entries: vec![1, 2, 3, 4],
        indices: vec![0, 1, 2, 3],
    };

    test_instance.erase_indices(0, 4);
    assert_eq!(test_instance.indices.len(), 0);
}

#[test]
fn test_erase_indices_with_no_shift() {
    struct TestStruct {
        entries: Vec<u32>,
        indices: Vec<usize>,
    }

    let mut test_instance = TestStruct {
        entries: vec![1, 2, 3, 4, 5],
        indices: vec![0, 1, 2, 3, 4],
    };

    test_instance.erase_indices(0, 1);
    assert_eq!(test_instance.indices, vec![0, 1, 2, 3]);
}

#[test]
fn test_erase_indices_with_shift_right() {
    struct TestStruct {
        entries: Vec<u32>,
        indices: Vec<usize>,
    }

    let mut test_instance = TestStruct {
        entries: vec![1, 2, 3, 4, 5],
        indices: vec![0, 1, 2, 3, 4],
    };

    test_instance.erase_indices(2, 3);
    assert_eq!(test_instance.indices, vec![0, 1, 2, 3]);
}

