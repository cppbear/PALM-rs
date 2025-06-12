// Answer 0

#[test]
fn test_to_base_index_non_zero_sized() {
    use std::ptr::NonNull;
    use std::mem;

    struct Bucket<T> {
        ptr: NonNull<T>,
    }

    let data: [i32; 5] = [0, 1, 2, 3, 4];
    let bucket_ptr = NonNull::from(&data[2]); // Bucket pointing to the 3rd element
    let base_ptr = NonNull::from(&data[5]);  // Base pointing to the end of the array

    let bucket = Bucket { ptr: bucket_ptr };

    unsafe {
        assert_eq!(bucket.to_base_index(base_ptr), 2);
    }
}

#[test]
fn test_to_base_index_zero_sized() {
    use std::ptr::NonNull;
    
    struct ZeroSizedBucket {
        ptr: NonNull<u8>,
    }

    let ptr = NonNull::new(0x1000 as *mut u8).unwrap(); // Dummy pointer for ZST case
    let bucket = ZeroSizedBucket { ptr };

    unsafe {
        assert_eq!(bucket.to_base_index(ptr), 0x1000 as usize - 1);
    }
}

