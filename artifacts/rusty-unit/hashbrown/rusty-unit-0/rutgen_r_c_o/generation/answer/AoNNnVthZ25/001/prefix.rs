// Answer 0

#[test]
fn test_new_in_with_valid_allocator() {
    use bumpalo::Bump;

    let bump = Bump::new();
    let table = HashTable::new_in(&bump);
}

#[test]
fn test_new_in_with_different_allocator() {
    use bumpalo::Bump;

    let bump = Bump::new();
    let table = HashTable::new_in(&bump);
}

#[test]
fn test_new_in_with_large_allocator() {
    use bumpalo::Bump;

    let bump = Bump::new();
    let table = HashTable::new_in(&bump);
}

#[test]
#[should_panic]
fn test_new_in_with_null_allocator() {
    let alloc: *const () = std::ptr::null();
    let _table = HashTable::new_in(unsafe { &*(alloc as *const Bump) });
}

