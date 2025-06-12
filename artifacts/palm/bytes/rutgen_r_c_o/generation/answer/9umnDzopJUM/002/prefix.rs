// Answer 0

#[test]
fn test_shallow_clone_vec_success() {
    use core::ptr::NonNull;
    use alloc::vec::Vec;
    
    let mut buffer: Vec<u8> = vec![0u8; 10];
    let buf_ptr = buffer.as_mut_ptr();
    let atom_ptr: *const () = NonNull::from(&buffer).as_ptr();
    let atom = AtomicPtr::new(atom_ptr);
    let offset: *const u8 = buf_ptr;
    let len: usize = 5;

    unsafe {
        shallow_clone_vec(&atom, atom_ptr, buf_ptr, offset, len);
    }
}

#[test]
fn test_shallow_clone_vec_success_min_len() {
    use core::ptr::NonNull;
    use alloc::vec::Vec;
    
    let mut buffer: Vec<u8> = vec![0u8; 1];
    let buf_ptr = buffer.as_mut_ptr();
    let atom_ptr: *const () = NonNull::from(&buffer).as_ptr();
    let atom = AtomicPtr::new(atom_ptr);
    let offset: *const u8 = buf_ptr;
    let len: usize = 1;

    unsafe {
        shallow_clone_vec(&atom, atom_ptr, buf_ptr, offset, len);
    }
}

#[test]
fn test_shallow_clone_vec_success_max_len() {
    use core::ptr::NonNull;
    use alloc::vec::Vec;

    let max_len: usize = usize::MAX - 1; // to stay within valid range
    let mut buffer: Vec<u8> = vec![0u8; max_len];
    let buf_ptr = buffer.as_mut_ptr();
    let atom_ptr: *const () = NonNull::from(&buffer).as_ptr();
    let atom = AtomicPtr::new(atom_ptr);
    let offset: *const u8 = buf_ptr;
    let len: usize = max_len;

    unsafe {
        shallow_clone_vec(&atom, atom_ptr, buf_ptr, offset, len);
    }
}

#[test]
fn test_shallow_clone_vec_concurrent_case() {
    use core::ptr::NonNull;
    use alloc::vec::Vec;
    
    let mut buffer: Vec<u8> = vec![0u8; 10];
    let buf_ptr = buffer.as_mut_ptr();
    let atom_ptr: *const () = NonNull::from(&buffer).as_ptr();
    let atom = AtomicPtr::new(atom_ptr);
    let offset: *const u8 = buf_ptr;
    let len: usize = 5;

    unsafe {
        // Simulating concurrent modifications by modifying the atom's pointer
        atom.store(NonNull::dangling().as_ptr(), Ordering::Relaxed);
        shallow_clone_vec(&atom, atom_ptr, buf_ptr, offset, len);
    }
}

#[test]
#[should_panic]
fn test_shallow_clone_vec_invalid_alignment() {
    use core::ptr::NonNull;
    use alloc::vec::Vec;
    
    let mut buffer: Vec<u8> = vec![0u8; 10];
    let buf_ptr = buffer.as_mut_ptr();
    let atom_ptr: *const () = NonNull::from(&buffer).as_ptr();
    let atom = AtomicPtr::new(atom_ptr);
    let offset: *const u8 = buf_ptr.offset(1); // deliberately misaligned
    let len: usize = 5;

    unsafe {
        shallow_clone_vec(&atom, atom_ptr, buf_ptr, offset, len);
    }
}

