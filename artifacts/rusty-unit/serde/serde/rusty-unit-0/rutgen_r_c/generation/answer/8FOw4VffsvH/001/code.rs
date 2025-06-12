// Answer 0

#[test]
fn test_map_deserializer_new() {
    struct TestPair;

    impl private::Pair for TestPair {}

    let vec_of_pairs = vec![TestPair, TestPair];
    let iter = vec_of_pairs.into_iter();

    let deserializer = MapDeserializer::new(iter);

    assert_eq!(std::mem::size_of_val(&deserializer.iter), std::mem::size_of::<_>());
    assert!(deserializer.value.is_none());
    assert_eq!(deserializer.count, 0);
}

#[test]
fn test_map_deserializer_fuse_functionality() {
    struct TestPair;

    impl private::Pair for TestPair {}

    let vec_of_pairs = vec![TestPair, TestPair];
    let iter = vec_of_pairs.into_iter();

    let deserializer = MapDeserializer::new(iter);
    
    // Ensure that the internal iterator is indeed a Fuse iterator
    assert!(deserializer.iter.is_fused());
}

#[test]
fn test_map_deserializer_lifetime_phantom() {
    struct TestPair;

    impl private::Pair for TestPair {}

    let vec_of_pairs = vec![TestPair, TestPair];
    let iter = vec_of_pairs.into_iter();

    let deserializer = MapDeserializer::new(iter);

    // Check that the lifetime is correctly represented (i.e., not a zero-sized type issue)
    let _: &PhantomData<()> = &deserializer.lifetime;
}

