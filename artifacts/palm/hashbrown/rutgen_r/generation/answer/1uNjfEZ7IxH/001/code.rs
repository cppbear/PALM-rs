// Answer 0

#[test]
fn test_get_many_mut_pointers_normal_case() {
    struct TestStruct {
        value: u64,
    }

    impl TestStruct {
        fn new(value: u64) -> Self {
            TestStruct { value }
        }
    }

    let mut data = vec![
        TestStruct::new(1),
        TestStruct::new(2),
        TestStruct::new(3),
    ];

    let hashes = [1, 2, 3];
    let mut eq = |index: usize, item: &TestStruct| item.value == hashes[index];

    let result: [Option<std::ptr::NonNull<TestStruct>>; 3] = unsafe {
        get_many_mut_pointers(&mut data, hashes, eq)
    };

    assert_eq!(result[0].is_some(), true);
    assert_eq!(result[1].is_some(), true);
    assert_eq!(result[2].is_some(), true);
}

#[test]
fn test_get_many_mut_pointers_empty_case() {
    struct TestStruct {
        value: u64,
    }

    let mut data: Vec<TestStruct> = Vec::new();

    let hashes = [1, 2, 3];
    let mut eq = |index: usize, _: &TestStruct| false;

    let result: [Option<std::ptr::NonNull<TestStruct>>; 3] = unsafe {
        get_many_mut_pointers(&mut data, hashes, eq)
    };

    assert!(result.iter().all(|&x| x.is_none()));
}

#[test]
#[should_panic]
fn test_get_many_mut_pointers_panic_condition() {
    struct TestStruct {
        value: u64,
    }

    let mut data = vec![
        TestStruct::new(1),
        TestStruct::new(2),
        TestStruct::new(3),
    ];

    let hashes = [1, 2, 3, 4];
    let mut eq = |index: usize, item: &TestStruct| item.value == hashes[index];

    let _result: [Option<std::ptr::NonNull<TestStruct>>; 4] = unsafe {
        get_many_mut_pointers(&mut data, hashes, eq)
    };
}

