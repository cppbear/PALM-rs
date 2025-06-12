// Answer 0

#[test]
fn test_from_boxed() {
    struct Bucket<K, V> {
        key: K,
        value: V,
    }

    struct MyMap {
        buckets: Box<[Bucket<i32, String>]>,
    }

    let entries: Box<[Bucket<i32, String>]> = Box::new([
        Bucket { key: 1, value: "one".to_string() },
        Bucket { key: 2, value: "two".to_string() },
    ]);

    let my_map: Box<MyMap> = from_boxed(entries);

    assert_eq!(my_map.buckets.len(), 2);
    assert_eq!(my_map.buckets[0].key, 1);
    assert_eq!(my_map.buckets[0].value, "one");
    assert_eq!(my_map.buckets[1].key, 2);
    assert_eq!(my_map.buckets[1].value, "two");
}

