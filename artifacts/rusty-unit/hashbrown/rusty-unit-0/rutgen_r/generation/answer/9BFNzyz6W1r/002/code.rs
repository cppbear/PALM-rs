// Answer 0

#[test]
fn test_to_base_index_non_zst_valid_input() {
    use std::ptr::NonNull;
    use std::mem::size_of;

    struct NonZST {
        dummy: u8,
    }

    struct Bucket {
        ptr: NonNull<NonZST>,
    }

    // Creating a RawTable-like environment
    const TABLE_SIZE: usize = 10;
    let mut table: Vec<NonZST> = Vec::with_capacity(TABLE_SIZE);
    for _ in 0..TABLE_SIZE {
        table.push(NonZST { dummy: 0 });
    }
    
    // Create a NonNull pointer to the second element
    let base_ptr = NonNull::new(table.as_mut_ptr().add(1)).unwrap();
    let bucket_ptr = NonNull::new(table.as_mut_ptr().add(5)).unwrap();
    
    let bucket = Bucket { ptr: bucket_ptr };

    // Calculate the expected index
    let expected_index = (bucket.ptr.as_ptr() as usize - base_ptr.as_ptr() as usize) / size_of::<NonZST>();
    let index = unsafe { bucket.to_base_index(base_ptr) };
    
    assert_eq!(index, expected_index);
}

#[test]
#[should_panic]
fn test_to_base_index_non_zst_dangling_base_pointer() {
    use std::ptr::NonNull;

    struct NonZST {
        dummy: u8,
    }

    struct Bucket {
        ptr: NonNull<NonZST>,
    }

    let bucket_ptr = NonNull::new(0 as *mut NonZST).unwrap();
    let bucket = Bucket { ptr: bucket_ptr };

    let base_ptr = NonNull::new(0 as *mut NonZST).unwrap();
    
    unsafe {
        bucket.to_base_index(base_ptr);
    }
}

#[test]
fn test_to_base_index_non_zst_edge_case() {
    use std::ptr::NonNull;
    use std::mem::size_of;

    struct NonZST {
        dummy: u8,
    }

    struct Bucket {
        ptr: NonNull<NonZST>,
    }

    const TABLE_SIZE: usize = 1;
    let mut table: Vec<NonZST> = Vec::with_capacity(TABLE_SIZE);
    table.push(NonZST { dummy: 0 });
    
    let base_ptr = NonNull::new(table.as_mut_ptr()).unwrap();
    let bucket_ptr = NonNull::new(table.as_mut_ptr()).unwrap();
    
    let bucket = Bucket { ptr: bucket_ptr };

    // With a single element, expect it to correctly identify the index
    let expected_index = (bucket.ptr.as_ptr() as usize - base_ptr.as_ptr() as usize) / size_of::<NonZST>();
    let index = unsafe { bucket.to_base_index(base_ptr) };
    
    assert_eq!(index, expected_index);
}

