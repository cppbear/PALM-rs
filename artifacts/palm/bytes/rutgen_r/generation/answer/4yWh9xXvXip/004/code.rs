// Answer 0

#[test]
#[should_panic(expected = "`len` greater than remaining")]
fn test_copy_to_bytes_panic() {
    struct TestStruct {
        a: crate::BytesMut,
        b: crate::BytesMut,
    }

    let mut a = crate::BytesMut::with_capacity(0); // a_rem == 0
    let mut b = crate::BytesMut::with_capacity(5); // b.remaining() = 5
    
    // Fill b to ensure it has data
    b.extend_from_slice(&[1, 2, 3, 4, 5]);

    let mut test_struct = TestStruct { a, b };

    // len = 10 (a_rem = 0, b.remaining() = 5, so len - a_rem > self.b.remaining())
    let len = 10;

    // This should trigger a panic
    let _ = test_struct.copy_to_bytes(len);
}

