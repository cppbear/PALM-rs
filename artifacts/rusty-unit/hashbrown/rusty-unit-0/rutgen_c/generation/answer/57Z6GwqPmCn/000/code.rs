// Answer 0

#[test]
fn test_write() {
    use std::ptr::null_mut;

    struct TestType {
        value: i32,
    }

    impl TestType {
        const IS_ZERO_SIZED: bool = false;
    }

    let mut value = TestType { value: 42 };
    let base_ptr = NonNull::new(&mut value).unwrap();
    let bucket = Bucket::<TestType>::from_base_index(base_ptr, 0);

    unsafe {
        bucket.write(TestType { value: 100 });
    }

    assert_eq!(value.value, 100);
}

#[test]
#[should_panic]
fn test_write_zero_sized() {
    struct ZST;

    impl ZST {
        const IS_ZERO_SIZED: bool = true;
    }

    let base_ptr = NonNull::<ZST>::new(null_mut()).unwrap();
    let bucket = Bucket::<ZST>::from_base_index(base_ptr, 0);

    unsafe {
        bucket.write(ZST);
    }
}

#[test]
fn test_write_multiple() {
    struct MultipleType {
        value: i32,
    }

    impl MultipleType {
        const IS_ZERO_SIZED: bool = false;
    }

    let mut values = vec![MultipleType { value: 1 }, MultipleType { value: 2 }];
    let base_ptr = NonNull::new(values.as_mut_ptr()).unwrap();
    let bucket = Bucket::<MultipleType>::from_base_index(base_ptr, 1);

    unsafe {
        bucket.write(MultipleType { value: 99 });
    }

    assert_eq!(values[1].value, 99);
}

