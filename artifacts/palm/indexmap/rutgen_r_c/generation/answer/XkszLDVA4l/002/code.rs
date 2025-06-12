// Answer 0

#[test]
#[should_panic(expected = "index out of bounds: the len is 0 but the index is 1. Expected index <= len")]
fn test_split_off_panic_out_of_bounds() {
    struct TestBucket {
        hash: HashValue,
        key: usize,
        value: usize,
    }

    let mut index_map = IndexMapCore::new();
    index_map.entries.push(TestBucket { hash: HashValue::default(), key: 1, value: 10 });
    index_map.entries.push(TestBucket { hash: HashValue::default(), key: 2, value: 20 });

    index_map.split_off(3);
}

#[test]
#[should_panic(expected = "index out of bounds: the len is 1 but the index is 2. Expected index <= len")]
fn test_split_off_panic_exact_length() {
    struct TestBucket {
        hash: HashValue,
        key: usize,
        value: usize,
    }

    let mut index_map = IndexMapCore::new();
    index_map.entries.push(TestBucket { hash: HashValue::default(), key: 1, value: 10 });

    index_map.split_off(2);
}

#[test]
fn test_split_off_success() {
    struct TestBucket {
        hash: HashValue,
        key: usize,
        value: usize,
    }

    let mut index_map = IndexMapCore::new();
    index_map.entries.push(TestBucket { hash: HashValue::default(), key: 1, value: 10 });
    index_map.entries.push(TestBucket { hash: HashValue::default(), key: 2, value: 20 });

    let new_map = index_map.split_off(1);

    assert_eq!(new_map.entries.len(), 1);
    assert_eq!(index_map.entries.len(), 1);
}

#[test]
fn test_split_off_empty() {
    struct TestBucket {
        hash: HashValue,
        key: usize,
        value: usize,
    }

    let mut index_map: IndexMapCore<usize, usize> = IndexMapCore::new();

    let new_map = index_map.split_off(0);

    assert_eq!(new_map.entries.len(), 0);
    assert_eq!(index_map.entries.len(), 0);
}

