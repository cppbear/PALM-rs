// Answer 0

#[test]
fn test_index_mut_valid_index() {
    struct TestBucket {
        hash: u64,
        key: usize,
        value: i32,
    }

    let mut slice = Slice {
        entries: [
            Bucket { hash: 0, key: 0, value: 10 },
            Bucket { hash: 1, key: 1, value: 20 },
            Bucket { hash: 2, key: 2, value: 30 },
        ],
    };

    let value: &mut i32 = slice.index_mut(1);
    *value += 5;

    assert_eq!(slice.entries[1].value, 25);
}

#[test]
#[should_panic]
fn test_index_mut_out_of_bounds() {
    struct TestBucket {
        hash: u64,
        key: usize,
        value: i32,
    }

    let mut slice = Slice {
        entries: [
            Bucket { hash: 0, key: 0, value: 10 },
            Bucket { hash: 1, key: 1, value: 20 },
        ],
    };

    let _value: &mut i32 = slice.index_mut(2);
}

