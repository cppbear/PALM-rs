// Answer 0

#[cfg(test)]
fn test_to_base_index() {
    struct ZST; // Zero-sized type
    impl ZST {
        const IS_ZERO_SIZED: bool = true;
    }

    struct NonZST {
        _data: u8,
    }
    impl NonZST {
        const IS_ZERO_SIZED: bool = false;
    }

    unsafe {
        // Creating non-null pointers for testing
        let mut data: [NonZST; 10] = std::mem::MaybeUninit::uninit().assume_init();
        let base_ptr = NonNull::new(&mut data[0]).unwrap();
        let bucket_ptr = Bucket { ptr: NonNull::new(&mut data[5]).unwrap() };

        // Test for NonZST
        assert_eq!(bucket_ptr.to_base_index(base_ptr), 5);

        // Test for ZST
        let mut zst_data: [ZST; 10] = std::mem::MaybeUninit::uninit().assume_init();
        let zst_base_ptr = NonNull::new(&mut zst_data[0]).unwrap();
        let zst_bucket_ptr = Bucket { ptr: NonNull::new(&mut zst_data[0]).unwrap() }; // points to the first ZST

        assert_eq!(zst_bucket_ptr.to_base_index(zst_base_ptr), 0);
    }
}

#[test]
fn test_bucket_to_base_index() {
    test_to_base_index();
}

