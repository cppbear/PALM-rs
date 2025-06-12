// Answer 0

#[test]
fn test_muts_valid_key_value() {
    let mut bucket = Bucket {
        hash: HashValue(1),
        key: 10,
        value: 20,
    };
    let (key_mut, value_mut) = bucket.muts();
}

#[test]
fn test_muts_minimum_key_value() {
    let mut bucket = Bucket {
        hash: HashValue(0),
        key: 0,
        value: 0,
    };
    let (key_mut, value_mut) = bucket.muts();
}

#[test]
fn test_muts_large_key_value() {
    let mut bucket = Bucket {
        hash: HashValue(1000),
        key: 999,
        value: 999,
    };
    let (key_mut, value_mut) = bucket.muts();
}

#[test]
fn test_muts_edge_case_key() {
    let mut bucket = Bucket {
        hash: HashValue(500),
        key: 1,
        value: 1000,
    };
    let (key_mut, value_mut) = bucket.muts();
}

#[test]
fn test_muts_edge_case_value() {
    let mut bucket = Bucket {
        hash: HashValue(250),
        key: 500,
        value: 1,
    };
    let (key_mut, value_mut) = bucket.muts();
}

