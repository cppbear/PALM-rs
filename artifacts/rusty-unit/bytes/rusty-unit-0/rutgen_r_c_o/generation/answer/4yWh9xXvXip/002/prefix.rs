// Answer 0

#[test]
fn test_copy_to_bytes_a_not_empty_b_remaining_large_enough() {
    let mut a = BytesMut::with_capacity(10);
    a.extend_from_slice(b"abcdefghij");
    let mut b = BytesMut::with_capacity(20);
    b.extend_from_slice(b"klmnopqrstuvwxy");
    
    let mut chain = Chain { a, b };
    chain.copy_to_bytes(15);
}

#[test]
fn test_copy_to_bytes_a_not_empty_b_remaining_exact() {
    let mut a = BytesMut::with_capacity(5);
    a.extend_from_slice(b"abcde");
    let mut b = BytesMut::with_capacity(10);
    b.extend_from_slice(b"1234567890");

    let mut chain = Chain { a, b };
    chain.copy_to_bytes(10);
}

#[test]
fn test_copy_to_bytes_a_empty_b_remaining() {
    let mut a = BytesMut::with_capacity(0);
    let mut b = BytesMut::with_capacity(20);
    b.extend_from_slice(b"abcdefghijabcdefghij");
    
    let mut chain = Chain { a, b };
    chain.copy_to_bytes(15);
}

#[test]
fn test_copy_to_bytes_a_not_empty_b_remaining_not_enough() {
    let mut a = BytesMut::with_capacity(0);
    let mut b = BytesMut::with_capacity(30);
    b.extend_from_slice(b"abcdefghijabcdefghijabcdefghijabcdefghij");
    
    let mut chain = Chain { a, b };
    chain.copy_to_bytes(15); 
}

