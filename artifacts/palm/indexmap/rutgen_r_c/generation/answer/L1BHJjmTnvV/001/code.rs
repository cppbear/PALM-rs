// Answer 0

#[test]
fn test_first_non_empty_slice() {
    struct TestBucket {
        hash: u64,
        key: i32,
        value: String,
    }
    
    let entry1 = TestBucket { hash: 1, key: 10, value: String::from("value1") };
    let entry2 = TestBucket { hash: 2, key: 20, value: String::from("value2") };

    let slice = Slice { entries: [entry1, entry2] };

    let result = slice.first();
    assert!(result.is_some());
    let (key, value) = result.unwrap();
    assert_eq!(*key, 10);
    assert_eq!(*value, "value1");
}

#[test]
fn test_first_empty_slice() {
    struct TestBucket {
        hash: u64,
        key: i32,
        value: String,
    }
    
    let slice: Slice<i32, String> = Slice { entries: [] };

    let result = slice.first();
    assert!(result.is_none());
}

#[test]
fn test_first_single_element_slice() {
    struct TestBucket {
        hash: u64,
        key: i32,
        value: String,
    }

    let entry = TestBucket { hash: 1, key: 30, value: String::from("value3") };
    let slice = Slice { entries: [entry] };

    let result = slice.first();
    assert!(result.is_some());
    let (key, value) = result.unwrap();
    assert_eq!(*key, 30);
    assert_eq!(*value, "value3");
}

