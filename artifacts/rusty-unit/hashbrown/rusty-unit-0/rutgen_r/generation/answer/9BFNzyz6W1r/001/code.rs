// Answer 0

#[test]
fn test_to_base_index_zst() {
    use std::mem::transmute;
    use std::ptr::NonNull;

    struct Bucket {
        ptr: NonNull<u8>,
    }

    impl Bucket {
        const IS_ZERO_SIZED: bool = true;

        unsafe fn to_base_index(&self, base: NonNull<u8>) -> usize {
            if Self::IS_ZERO_SIZED {
                self.ptr.as_ptr() as usize - 1
            } else {
                std::ptr::offset_from(base.as_ptr(), self.ptr.as_ptr())
            }
        }
    }

    let base_value = 0x1000 as *mut u8;
    
    let bucket_ptr = base_value;
    let bucket = Bucket {
        ptr: NonNull::new(bucket_ptr).unwrap(),
    };

    let base = NonNull::new(base_value).unwrap();
    
    let result = unsafe { bucket.to_base_index(base) };
    assert_eq!(result, 0x1000_usize - 1);
}

