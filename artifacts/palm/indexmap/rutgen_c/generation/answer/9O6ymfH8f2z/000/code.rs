// Answer 0

#[test]
fn test_keys_empty() {
    struct TestMap {
        entries: [Bucket<i32, i32>; 0],
    }

    let map = Slice { entries: [] };
    let keys = map.keys();
    assert_eq!(keys.iter.len(), 0);
}

#[test]
fn test_keys_non_empty() {
    struct TestMap {
        entries: [Bucket<i32, i32>; 2],
    }

    let map = Slice {
        entries: [
            Bucket { hash: 0, key: 1, value: 10 },
            Bucket { hash: 1, key: 2, value: 20 },
        ],
    };
    
    let keys = map.keys();
    let mut keys_vec: Vec<_> = keys.iter.map(|bucket| &bucket.key).collect();
    assert_eq!(keys_vec, vec![&1, &2]);
}

#[test]
fn test_keys_multiple() {
    struct TestMap {
        entries: [Bucket<i32, i32>; 3],
    }

    let map = Slice {
        entries: [
            Bucket { hash: 0, key: 3, value: 30 },
            Bucket { hash: 1, key: 4, value: 40 },
            Bucket { hash: 2, key: 5, value: 50 },
        ],
    };

    let keys = map.keys();
    let mut keys_vec: Vec<_> = keys.iter.map(|bucket| &bucket.key).collect();
    assert_eq!(keys_vec, vec![&3, &4, &5]);
}

