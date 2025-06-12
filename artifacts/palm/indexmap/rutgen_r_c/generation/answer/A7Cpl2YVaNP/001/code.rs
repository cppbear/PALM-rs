// Answer 0

#[test]
fn test_into_boxed_with_non_empty_slice() {
    let buckets: Box<[Bucket<&str, i32>]> = Box::new([
        Bucket { hash: 1, key: "a", value: 10 },
        Bucket { hash: 2, key: "b", value: 20 },
    ]);
    
    let slice: Box<Slice<&str, i32>> = Box::new(Slice { entries: *buckets });
    let boxed_result: Box<[Bucket<&str, i32>]> = slice.into_boxed();

    assert_eq!(boxed_result.len(), 2);
    assert_eq!(boxed_result[0].key, "a");
    assert_eq!(boxed_result[0].value, 10);
    assert_eq!(boxed_result[1].key, "b");
    assert_eq!(boxed_result[1].value, 20);
}

#[test]
fn test_into_boxed_with_empty_slice() {
    let buckets: Box<[Bucket<&str, i32>]> = Box::new([]);
    let slice: Box<Slice<&str, i32>> = Box::new(Slice { entries: *buckets });
    let boxed_result: Box<[Bucket<&str, i32>]> = slice.into_boxed();

    assert_eq!(boxed_result.len(), 0);
}

#[should_panic]
fn test_into_boxed_with_panic_condition() {
    let buckets: Box<[Bucket<&str, i32>]> = Box::new([
        Bucket { hash: 1, key: "a", value: 10 },
    ]);
    
    let slice: Box<Slice<&str, i32>> = Box::new(Slice { entries: *buckets });
    
    // Manual panic condition: dropping the slice early to cause a double free.
    drop(slice);
    let _boxed_result: Box<[Bucket<&str, i32>]> = slice.into_boxed();
}

