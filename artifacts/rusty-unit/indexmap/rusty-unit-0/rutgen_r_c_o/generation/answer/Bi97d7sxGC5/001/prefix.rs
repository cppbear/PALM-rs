// Answer 0

#[test]
fn test_into_boxed_non_empty() {
    let buckets: Box<[Bucket<i32>]> = Box::new([
        Bucket { hash: HashValue, key: 1, value: 2 },
        Bucket { hash: HashValue, key: 3, value: 4 },
    ]);
    let slice = Box::new(Slice { entries: buckets });
    let _result: Box<[Bucket<i32>]> = slice.into_boxed();
}

#[test]
fn test_into_boxed_multiple_elements() {
    let buckets: Box<[Bucket<i32>]> = Box::new([
        Bucket { hash: HashValue, key: 1, value: 2 },
        Bucket { hash: HashValue, key: 3, value: 4 },
        Bucket { hash: HashValue, key: 5, value: 6 },
    ]);
    let slice = Box::new(Slice { entries: buckets });
    let _result: Box<[Bucket<i32>]> = slice.into_boxed();
}

#[test]
fn test_into_boxed_large_array() {
    let mut entries = Vec::with_capacity(1000);
    for i in 0..1000 {
        entries.push(Bucket { hash: HashValue, key: i, value: i * 2 });
    }
    let buckets = entries.into_boxed_slice();
    let slice = Box::new(Slice { entries: buckets });
    let _result: Box<[Bucket<i32>]> = slice.into_boxed();
}

#[should_panic]
fn test_into_boxed_empty_slice() {
    let buckets: Box<[Bucket<i32>]> = Box::new([]);
    let slice = Box::new(Slice { entries: buckets });
    let _result: Box<[Bucket<i32>]> = slice.into_boxed();
}

