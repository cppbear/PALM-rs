// Answer 0

#[test]
fn test_promotable_even_to_mut_with_arc() {
    use core::ptr::NonNull;
    use alloc::alloc::{alloc, dealloc, Layout};
    use alloc::vec::Vec;
    use crate::bytes::BytesMut;
    
    let layout = Layout::from_size_align(32, 8).unwrap();
    unsafe {
        let ptr = alloc(layout);
        let atomic_ptr = AtomicPtr::new(ptr);
        
        let len = 16;
        
        let result = promotable_even_to_mut(&atomic_ptr, ptr, len);
        
        assert_eq!(result.len(), len); // Check the length of the resulting BytesMut

        dealloc(ptr, layout);
    }
}

#[test]
fn test_promotable_even_to_mut_with_vector() {
    use core::ptr::NonNull;
    use alloc::alloc::{alloc, dealloc, Layout};
    use alloc::vec::Vec;
    use crate::bytes::BytesMut;
    
    let layout = Layout::from_size_align(32, 8).unwrap();
    unsafe {
        let ptr = alloc(layout);
        let atomic_ptr = AtomicPtr::new(ptr);

        // Simulating vector-based memory allocation
        let vec: Vec<u8> = Vec::from_iter((0..16).map(|x| x as u8));
        let vec_ptr = vec.as_ptr();
        let len = vec.len();
        
        let result = promotable_even_to_mut(&atomic_ptr, vec_ptr, len);
        
        assert_eq!(result.len(), len); // Check the length of the resulting BytesMut
        
        dealloc(ptr, layout);
    }
}

