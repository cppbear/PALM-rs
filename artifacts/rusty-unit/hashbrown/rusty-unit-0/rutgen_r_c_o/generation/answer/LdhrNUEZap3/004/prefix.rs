// Answer 0

#[test]
fn test_fold_impl_with_valid_inputs() {
    struct MyStruct {
        value: i32,
    }

    #[unsafe]
    fn fold_fn(acc: i32, bucket: Bucket<MyStruct>) -> i32 {
        acc + unsafe { bucket.as_ref() }.value
    }

    let data = Bucket {
        ptr: NonNull::new_unchecked(std::ptr::NonNull::new_unchecked(std::ptr::null_mut::<MyStruct>()))
    };

    unsafe {
        let mut iterator = RawIterRange::new(std::ptr::null(), data, 4);
        let result = iterator.fold_impl(4, 0, fold_fn);
    }
}

#[test]
#[should_panic]
fn test_fold_impl_should_panic_when_n_is_zero() {
    struct MyStruct {
        value: i32,
    }

    #[unsafe]
    fn fold_fn(acc: i32, bucket: Bucket<MyStruct>) -> i32 {
        acc + unsafe { bucket.as_ref() }.value
    }

    let data = Bucket {
        ptr: NonNull::new_unchecked(std::ptr::NonNull::new_unchecked(std::ptr::null_mut::<MyStruct>()))
    };

    unsafe {
        let mut iterator = RawIterRange::new(std::ptr::null(), data, 4);
        let result = iterator.fold_impl(0, 0, fold_fn);
    }
}

