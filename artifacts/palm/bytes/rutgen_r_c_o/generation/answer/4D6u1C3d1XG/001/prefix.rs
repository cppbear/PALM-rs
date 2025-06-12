// Answer 0

#[test]
fn test_split_to_empty() {
    let mut bytes_mut = BytesMut::new();
    let result = bytes_mut.split_to(0);
}

#[test]
fn test_split_to_length_one() {
    let mut bytes_mut = BytesMut::with_capacity(1);
    unsafe {
        bytes_mut.set_len(1);
    }
    let result = bytes_mut.split_to(1);
}

#[test]
fn test_split_to_exact_length() {
    let mut bytes_mut = BytesMut::with_capacity(5);
    unsafe {
        bytes_mut.set_len(5);
    }
    let result = bytes_mut.split_to(5);
}

#[test]
fn test_split_to_non_empty() {
    let mut bytes_mut = BytesMut::from_vec(vec![1, 2, 3, 4, 5]);
    let result = bytes_mut.split_to(3);
}

#[should_panic]
fn test_split_to_out_of_bounds() {
    let mut bytes_mut = BytesMut::new();
    let _ = bytes_mut.split_to(1);
}

#[should_panic]
fn test_split_to_exceeding_length() {
    let mut bytes_mut = BytesMut::with_capacity(2);
    unsafe {
        bytes_mut.set_len(2);
    }
    let _ = bytes_mut.split_to(3);
}

