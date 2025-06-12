// Answer 0

#[test]
fn test_promote_to_shared_with_valid_ref_cnt() {
    let mut bytes_mut = unsafe { BytesMut::with_capacity(10) };
    unsafe {
        bytes_mut.data = (KIND_VEC as *mut Shared).cast();
        bytes_mut.len = 5;
        bytes_mut.cap = 10;
    }
    
    unsafe {
        bytes_mut.promote_to_shared(1);
    }

    assert!(bytes_mut.data as usize & KIND_MASK == KIND_ARC);
}

#[test]
#[should_panic(expected = "assertion failed: self.kind() == KIND_VEC")]
fn test_promote_to_shared_with_incorrect_kind() {
    let mut bytes_mut = unsafe { BytesMut::with_capacity(10) };
    unsafe {
        bytes_mut.data = (KIND_ARC as *mut Shared).cast();
        bytes_mut.len = 5;
        bytes_mut.cap = 10;
    }

    unsafe {
        bytes_mut.promote_to_shared(1);
    }
}

#[test]
#[should_panic(expected = "assertion failed: ref_cnt == 1 || ref_cnt == 2")]
fn test_promote_to_shared_with_invalid_ref_cnt() {
    let mut bytes_mut = unsafe { BytesMut::with_capacity(10) };
    unsafe {
        bytes_mut.data = (KIND_VEC as *mut Shared).cast();
        bytes_mut.len = 5;
        bytes_mut.cap = 10;
    }

    unsafe {
        bytes_mut.promote_to_shared(3);
    }
}

#[test]
fn test_promote_to_shared_with_valid_fixture() {
    let mut bytes_mut = unsafe { BytesMut::with_capacity(20) };
    unsafe {
        bytes_mut.data = (KIND_VEC as *mut Shared).cast();
        bytes_mut.len = 10;
        bytes_mut.cap = 20;
    }

    unsafe {
        bytes_mut.promote_to_shared(2);
    }

    assert!(bytes_mut.data as usize & KIND_MASK == KIND_ARC);
}

