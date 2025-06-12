// Answer 0

#[test]
fn test_third_with_various_types() {
    let tuple_1 = (1, "two", "three");
    assert_eq!(third(tuple_1), "three");

    let tuple_2 = (true, 3.14, 'c');
    assert_eq!(third(tuple_2), 'c');

    let tuple_3 = (vec![1, 2, 3], "text", Some(42));
    assert_eq!(third(tuple_3), Some(42));

    let tuple_4 = (0.1, 0.2, 0.3);
    assert_eq!(third(tuple_4), 0.3);

    let tuple_5 = ("first", "second", "third");
    assert_eq!(third(tuple_5), "third");
}

#[test]
fn test_third_with_empty_tuple_elements() {
    let tuple_6 = ((), (), ());
    assert_eq!(third(tuple_6), ());
}

#[test]
fn test_third_with_different_bounds() {
    let tuple_7: (Range<u32>, Range<u32>, Range<u32>) = (1..3, 3..5, 5..=7);
    assert_eq!(third(tuple_7), 5..=7);
}

