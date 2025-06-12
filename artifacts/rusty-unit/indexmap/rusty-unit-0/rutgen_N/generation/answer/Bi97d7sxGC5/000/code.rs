// Answer 0

#[test]
fn test_into_boxed() {
    struct Bucket<T> {
        value: T,
    }

    let bucket1 = Bucket { value: 1 };
    let bucket2 = Bucket { value: 2 };

    let buckets: Vec<Bucket<i32>> = vec![bucket1, bucket2];
    let boxed_buckets: Box<[Bucket<i32>]> = Box::new(buckets.into_boxed_slice());

    let result = boxed_buckets.into_boxed();

    assert_eq!(unsafe { &*result.as_ptr() }, &Bucket { value: 1 });
    assert_eq!(unsafe { &*result.as_ptr().add(1) }, &Bucket { value: 2 });
}

