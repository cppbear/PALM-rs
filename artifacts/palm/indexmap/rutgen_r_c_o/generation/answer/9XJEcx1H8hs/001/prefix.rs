// Answer 0

#[test]
fn split_first_mut_single_element() {
    let mut slice = Box::new(Slice {
        entries: [
            Bucket {
                hash: 0,
                key: 1,
                value: "single"
            },
        ],
    });
    let result = slice.split_first_mut();
}

#[test]
fn split_first_mut_multiple_elements() {
    let mut slice = Box::new(Slice {
        entries: [
            Bucket {
                hash: 0,
                key: 1,
                value: "first"
            },
            Bucket {
                hash: 1,
                key: 2,
                value: "second"
            },
        ],
    });
    let result = slice.split_first_mut();
}

#[test]
fn split_first_mut_multiple_elements_longer_slice() {
    let mut slice = Box::new(Slice {
        entries: [
            Bucket {
                hash: 0,
                key: 1,
                value: "first"
            },
            Bucket {
                hash: 1,
                key: 2,
                value: "second"
            },
            Bucket {
                hash: 2,
                key: 3,
                value: "third"
            },
        ],
    });
    let result = slice.split_first_mut();
}

#[test]
fn split_first_mut_large_slice() {
    let mut slice = Box::new(Slice {
        entries: [
            Bucket {
                hash: 0,
                key: 1,
                value: "first"
            },
            Bucket {
                hash: 1,
                key: 2,
                value: "second"
            },
            Bucket {
                hash: 2,
                key: 3,
                value: "third"
            },
            Bucket {
                hash: 3,
                key: 4,
                value: "fourth"
            },
            Bucket {
                hash: 4,
                key: 5,
                value: "fifth"
            },
        ],
    });
    let result = slice.split_first_mut();
}

#[test]
#[should_panic]
fn split_first_mut_empty_slice() {
    let mut slice = Box::new(Slice { entries: [] });
    let result = slice.split_first_mut();
}

