// Answer 0

#[test]
fn test_next_function_found() {
    struct TestStruct {
        iter: Vec<std::ptr::NonNull<i32>>,
        table: std::collections::HashMap<std::ptr::NonNull<i32>, (i32, )>,
    }

    let mut value1 = Box::new(1);
    let mut value2 = Box::new(2);
    let non_null1 = std::ptr::NonNull::new(&mut *value1).unwrap();
    let non_null2 = std::ptr::NonNull::new(&mut *value2).unwrap();

    let mut table = std::collections::HashMap::new();
    table.insert(non_null1, (1,));
    table.insert(non_null2, (2,));

    let mut test_struct = TestStruct {
        iter: vec![non_null1, non_null2],
        table,
    };

    let result = test_struct.next(|item| **item == 1);
    assert_eq!(result, Some(1));
    assert!(!test_struct.iter.contains(&non_null1));
}

#[test]
fn test_next_function_not_found() {
    struct TestStruct {
        iter: Vec<std::ptr::NonNull<i32>>,
        table: std::collections::HashMap<std::ptr::NonNull<i32>, (i32, )>,
    }

    let mut value1 = Box::new(1);
    let mut value2 = Box::new(2);
    let non_null1 = std::ptr::NonNull::new(&mut *value1).unwrap();
    let non_null2 = std::ptr::NonNull::new(&mut *value2).unwrap();

    let mut table = std::collections::HashMap::new();
    table.insert(non_null1, (1,));
    table.insert(non_null2, (2,));

    let mut test_struct = TestStruct {
        iter: vec![non_null1, non_null2],
        table,
    };

    let result = test_struct.next(|item| **item == 3);
    assert_eq!(result, None);
}

