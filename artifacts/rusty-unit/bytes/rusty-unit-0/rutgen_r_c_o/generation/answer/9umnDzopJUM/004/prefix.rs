// Answer 0

#[test]
fn test_shallow_clone_vec_zero_len() {
    let atom: AtomicPtr<()> = AtomicPtr::new(NonNull::dangling().as_ptr());
    let buf: Vec<u8> = vec![0; 10];
    let mut raw_buf = buf.as_mut_ptr();
    let offset = raw_buf.add(0);
    let len = 0;

    unsafe { shallow_clone_vec(&atom, atom.as_ptr(), raw_buf, offset, len) };
}

#[test]
fn test_shallow_clone_vec_small_len() {
    let atom: AtomicPtr<()> = AtomicPtr::new(NonNull::dangling().as_ptr());
    let buf: Vec<u8> = vec![0; 10];
    let mut raw_buf = buf.as_mut_ptr();
    let offset = raw_buf.add(2);
    let len = 5;

    unsafe { shallow_clone_vec(&atom, atom.as_ptr(), raw_buf, offset, len) };
}

#[test]
fn test_shallow_clone_vec_large_len() {
    let atom: AtomicPtr<()> = AtomicPtr::new(NonNull::dangling().as_ptr());
    let buf: Vec<u8> = vec![0; 100];
    let mut raw_buf = buf.as_mut_ptr();
    let offset = raw_buf.add(20);
    let len = 80;

    unsafe { shallow_clone_vec(&atom, atom.as_ptr(), raw_buf, offset, len) };
}

#[test]
fn test_shallow_clone_vec_edge_case() {
    let atom: AtomicPtr<()> = AtomicPtr::new(NonNull::dangling().as_ptr());
    let buf: Vec<u8> = vec![0; 1];
    let mut raw_buf = buf.as_mut_ptr();
    let offset = raw_buf.add(0);
    let len = 1;

    unsafe { shallow_clone_vec(&atom, atom.as_ptr(), raw_buf, offset, len) };
}

#[test]
#[should_panic]
fn test_shallow_clone_vec_exceeding_offset() {
    let atom: AtomicPtr<()> = AtomicPtr::new(NonNull::dangling().as_ptr());
    let buf: Vec<u8> = vec![0; 10];
    let mut raw_buf = buf.as_mut_ptr();
    let offset = raw_buf.add(15); // Invalid offset exceeding the buf
    let len = 5;

    unsafe { shallow_clone_vec(&atom, atom.as_ptr(), raw_buf, offset, len) };
}

