// Answer 0

#[test]
fn test_shared_v_to_vec_not_unique() {
    use std::sync::atomic::{AtomicPtr, Ordering};
    use std::ptr;
    use std::slice;
    use std::mem;

    struct Shared {
        vec: Vec<u8>,
        unique: bool,
    }

    impl Shared {
        fn is_unique(&self) -> bool {
            self.unique
        }
    }

    fn release_shared(_shared: *mut Shared) {}

    unsafe fn shared_v_to_vec(data: &AtomicPtr<()>, ptr: *const u8, len: usize) -> Vec<u8> {
        let shared: *mut Shared = data.load(Ordering::Relaxed).cast();

        if (*shared).is_unique() {
            let shared = &mut *shared;

            // Drop shared
            let mut vec = mem::replace(&mut shared.vec, Vec::new());
            release_shared(shared);

            // Copy back buffer
            ptr::copy(ptr, vec.as_mut_ptr(), len);
            vec.set_len(len);

            vec
        } else {
            let v = slice::from_raw_parts(ptr, len).to_vec();
            release_shared(shared);
            v
        }
    }

    let unique_data = Shared {
        vec: vec![1, 2, 3],
        unique: false,
    };

    let data = AtomicPtr::new(&unique_data as *const Shared as *mut Shared);
    
    let input_data: [u8; 5] = [4, 5, 6, 7, 8];
    let input_ptr = input_data.as_ptr();

    let result = unsafe { shared_v_to_vec(&data, input_ptr, input_data.len()) };

    assert_eq!(result, input_data.to_vec());
}

