// Answer 0

#[test]
fn test_split_off_full_capacity() {
    let mut bytes_mut = BytesMut::with_capacity(10);
    bytes_mut.resize(10, 0);
    let _other = bytes_mut.split_off(10);
}

#[test]
fn test_split_off_zero() {
    let mut bytes_mut = BytesMut::with_capacity(10);
    bytes_mut.resize(5, 0);
    let _other = bytes_mut.split_off(0);
}

#[test]
fn test_split_off_partial() {
    let mut bytes_mut = BytesMut::with_capacity(20);
    bytes_mut.resize(15, 0);
    let _other = bytes_mut.split_off(5);
}

#[test]
#[should_panic]
fn test_split_off_out_of_bounds() {
    let mut bytes_mut = BytesMut::with_capacity(10);
    bytes_mut.resize(5, 0);
    let _other = bytes_mut.split_off(6);
}

