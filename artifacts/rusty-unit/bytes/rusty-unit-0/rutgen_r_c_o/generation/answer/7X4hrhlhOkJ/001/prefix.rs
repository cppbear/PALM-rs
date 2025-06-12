// Answer 0

#[test]
fn test_kind_zero_data() {
    let mut bytes_mut = BytesMut {
        ptr: NonNull::new(0 as *mut u8).unwrap(),
        len: 0,
        cap: 0,
        data: &0 as *const _ as *mut Shared,
    };
    let result = bytes_mut.kind();
}

#[test]
fn test_kind_one_data() {
    let mut bytes_mut = BytesMut {
        ptr: NonNull::new(1 as *mut u8).unwrap(),
        len: 0,
        cap: 0,
        data: &1 as *const _ as *mut Shared,
    };
    let result = bytes_mut.kind();
}

