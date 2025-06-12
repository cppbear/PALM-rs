// Answer 0

#[test]
fn test_into_boxed_non_empty() {
    struct Bucket<K, V> {
        key: K,
        value: V,
    }

    let vec: Vec<Bucket<i32, &str>> = vec![
        Bucket { key: 1, value: "a" },
        Bucket { key: 2, value: "b" },
    ];
    let boxed = Box::new(vec);

    let boxed_array: Box<[Bucket<i32, &str>]> = boxed.into_boxed();

    assert_eq!(boxed_array.len(), 2);
    assert_eq!(boxed_array[0].key, 1);
    assert_eq!(boxed_array[0].value, "a");
    assert_eq!(boxed_array[1].key, 2);
    assert_eq!(boxed_array[1].value, "b");
}

#[test]
fn test_into_boxed_empty() {
    struct Bucket<K, V> {
        key: K,
        value: V,
    }

    let vec: Vec<Bucket<i32, &str>> = vec![];
    let boxed = Box::new(vec);

    let boxed_array: Box<[Bucket<i32, &str>]> = boxed.into_boxed();

    assert_eq!(boxed_array.len(), 0);
}

#[should_panic]
fn test_into_boxed_null() {
    struct Bucket<K, V> {
        key: K,
        value: V,
    }

    let boxed: Box<Vec<Bucket<i32, &str>>> = Box::new(Vec::new());
    let _null_array: Box<[Bucket<i32, &str>]> = unsafe { Box::from_raw(Box::into_raw(boxed) as *mut [Bucket<i32, &str>]) };
}

