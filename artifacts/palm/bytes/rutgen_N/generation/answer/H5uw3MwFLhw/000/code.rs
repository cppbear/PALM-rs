// Answer 0

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem::MaybeUninit;

    struct UninitSlice {
        data: Vec<MaybeUninit<u8>>,
    }

    impl UninitSlice {
        unsafe fn uninit(slice: &mut [MaybeUninit<u8>]) -> &mut Self {
            &mut *(slice as *mut _ as *mut Self)
        }

        fn len(&self) -> usize {
            self.data.len()
        }
    }

    #[test]
    fn test_from_raw_parts_mut() {
        let mut bytes = vec![0u8; 5]; // An array of 5 uninitialized bytes
        let ptr = bytes.as_mut_ptr();
        let len = bytes.len();
        
        let slice: &mut UninitSlice;

        unsafe {
            slice = UninitSlice::from_raw_parts_mut(ptr, len);
        }

        assert_eq!(slice.len(), len);
    }

    #[should_panic]
    fn test_from_raw_parts_mut_invalid_pointer() {
        let len = 5;
        let invalid_ptr: *mut u8 = std::ptr::null_mut();

        unsafe {
            UninitSlice::from_raw_parts_mut(invalid_ptr, len);
        }
    }
    
    #[test]
    fn test_from_raw_parts_mut_zero_length() {
        let mut bytes: Vec<MaybeUninit<u8>> = Vec::new(); // Empty slice
        let ptr = bytes.as_mut_ptr();
        let len = 0;

        let slice: &mut UninitSlice;

        unsafe {
            slice = UninitSlice::from_raw_parts_mut(ptr, len);
        }

        assert_eq!(slice.len(), len);
    }
}

