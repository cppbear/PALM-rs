// Answer 0

#[test]
fn test_to_base_index_zero_sized() {
    struct ZST; // Zero-sized type

    impl ZST {
        const IS_ZERO_SIZED: bool = true;
    }

    let vec = Vec::new();
    let ptr = NonNull::new(vec.as_ptr() as *mut ZST).unwrap(); // creating a NonNull pointer to ZST
    let bucket = Bucket { ptr };

    unsafe {
        let result = bucket.to_base_index(ptr);

        // since T is a zero-sized type, we expect the result to be ptr.as_ptr() as usize - 1
        assert_eq!(result, ptr.as_ptr() as usize - 1);
    }
}

#[test]
fn test_to_base_index_zero_sized_aligned() {
    struct ZST; // Zero-sized type

    impl ZST {
        const IS_ZERO_SIZED: bool = true;
    }

    let vec = vec![ZST; 10]; // Creating a vector with multiple ZSTs
    let ptr = NonNull::new(vec.as_ptr() as *mut ZST).unwrap(); // creating a NonNull pointer to ZST
    let bucket = Bucket { ptr };

    unsafe {
        let result = bucket.to_base_index(ptr);
        
        // since T is a zero-sized type, we expect the result to be ptr.as_ptr() as usize - 1
        assert_eq!(result, ptr.as_ptr() as usize - 1);
    }
}

