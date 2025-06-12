// Answer 0

#[test]
fn test_col() {
    struct TestStruct {
        col: usize,
    }

    let instance = TestStruct { col: 5 };
    assert_eq!(instance.col(), 5);
}

#[test]
fn test_col_zero() {
    struct TestStruct {
        col: usize,
    }

    let instance = TestStruct { col: 0 };
    assert_eq!(instance.col(), 0);
}

#[test]
fn test_col_large() {
    struct TestStruct {
        col: usize,
    }

    let instance = TestStruct { col: usize::MAX };
    assert_eq!(instance.col(), usize::MAX);
}

