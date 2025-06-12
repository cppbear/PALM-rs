// Answer 0

#[cfg(test)]
fn test_partition_point() {
    struct TestBucket {
        key: usize,
    }
    
    let entries = vec![
        Bucket { hash: 0, key: 1, value: TestBucket { key: 1 } },
        Bucket { hash: 0, key: 3, value: TestBucket { key: 3 } },
        Bucket { hash: 0, key: 5, value: TestBucket { key: 5 } },
    ];
    
    let slice = Slice { entries };

    let predicate = |&key: &usize| key < 4;
    
    assert_eq!(slice.partition_point(predicate), 2);
}

#[cfg(test)]
fn test_partition_point_empty() {
    let entries: Vec<Bucket<usize, usize>> = vec![];
    let slice = Slice { entries };

    let predicate = |&key: &usize| key < 1;

    assert_eq!(slice.partition_point(predicate), 0);
}

#[cfg(test)]
fn test_partition_point_all_true() {
    struct TestBucket {
        key: usize,
    }

    let entries = vec![
        Bucket { hash: 0, key: 1, value: TestBucket { key: 1 } },
        Bucket { hash: 0, key: 2, value: TestBucket { key: 2 } },
        Bucket { hash: 0, key: 3, value: TestBucket { key: 3 } },
    ];

    let slice = Slice { entries };

    let predicate = |&key: &usize| key < 4;

    assert_eq!(slice.partition_point(predicate), 3);
}

#[cfg(test)]
fn test_partition_point_all_false() {
    struct TestBucket {
        key: usize,
    }

    let entries = vec![
        Bucket { hash: 0, key: 1, value: TestBucket { key: 1 } },
        Bucket { hash: 0, key: 2, value: TestBucket { key: 2 } },
        Bucket { hash: 0, key: 3, value: TestBucket { key: 3 } },
    ];

    let slice = Slice { entries };

    let predicate = |&key: &usize| key > 3;

    assert_eq!(slice.partition_point(predicate), 0);
}

