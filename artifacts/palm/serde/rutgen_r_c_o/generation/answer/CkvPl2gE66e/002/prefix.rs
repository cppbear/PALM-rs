// Answer 0

#[test]
fn test_end_with_remaining_one_and_count_zero() {
    let iter = vec![(1, 2)].into_iter().fuse(); // remaining = 1
    let deserializer = MapDeserializer {
        iter,
        value: None,
        count: 0,
        lifetime: PhantomData,
        error: PhantomData,
    };
    // Call the end function here, expecting it to return an Err
    let _ = deserializer.end();
}

#[test]
fn test_end_with_remaining_ten_and_count_zero() {
    let iter = (0..10).map(|i| (i, i)).into_iter().fuse(); // remaining = 10
    let deserializer = MapDeserializer {
        iter,
        value: None,
        count: 0,
        lifetime: PhantomData,
        error: PhantomData,
    };
    // Call the end function here, expecting it to return an Err
    let _ = deserializer.end();
}

#[test]
fn test_end_with_remaining_ten_and_count_five() {
    let iter = (0..10).map(|i| (i, i)).into_iter().fuse(); // remaining = 10
    let deserializer = MapDeserializer {
        iter,
        value: None,
        count: 5,
        lifetime: PhantomData,
        error: PhantomData,
    };
    // Call the end function here, expecting it to return an Err
    let _ = deserializer.end();
}

#[test]
fn test_end_with_remaining_fifty_and_count_twenty() {
    let iter = (0..50).map(|i| (i, i)).into_iter().fuse(); // remaining = 50
    let deserializer = MapDeserializer {
        iter,
        value: None,
        count: 20,
        lifetime: PhantomData,
        error: PhantomData,
    };
    // Call the end function here, expecting it to return an Err
    let _ = deserializer.end();
}

#[test]
fn test_end_with_remaining_ninety_and_count_seventy() {
    let iter = (0..90).map(|i| (i, i)).into_iter().fuse(); // remaining = 90
    let deserializer = MapDeserializer {
        iter,
        value: None,
        count: 70,
        lifetime: PhantomData,
        error: PhantomData,
    };
    // Call the end function here, expecting it to return an Err
    let _ = deserializer.end();
}

#[test]
fn test_end_with_remaining_one_hundred_and_count_ninety_nine() {
    let iter = (0..100).map(|i| (i, i)).into_iter().fuse(); // remaining = 100
    let deserializer = MapDeserializer {
        iter,
        value: None,
        count: 99,
        lifetime: PhantomData,
        error: PhantomData,
    };
    // Call the end function here, expecting it to return an Err
    let _ = deserializer.end();
}

