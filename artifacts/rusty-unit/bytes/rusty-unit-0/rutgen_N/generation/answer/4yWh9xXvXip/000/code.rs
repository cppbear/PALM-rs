// Answer 0

#[test]
fn test_copy_to_bytes_a_rem_greater_than_len() {
    struct TestStruct {
        a: crate::BytesMut,
        b: crate::BytesMut,
    }

    let mut test_struct = TestStruct {
        a: crate::BytesMut::from("123456789"),
        b: crate::BytesMut::from("abcdefgh"),
    };

    let result = test_struct.copy_to_bytes(5);
    assert_eq!(result.len(), 5);
    assert_eq!(&*result, b"12345");
}

#[test]
fn test_copy_to_bytes_a_rem_equal_to_zero() {
    struct TestStruct {
        a: crate::BytesMut,
        b: crate::BytesMut,
    }

    let mut test_struct = TestStruct {
        a: crate::BytesMut::from(""),
        b: crate::BytesMut::from("abcdefgh"),
    };

    let result = test_struct.copy_to_bytes(5);
    assert_eq!(result.len(), 5);
    assert_eq!(&*result, b"abcde");
}

#[test]
fn test_copy_to_bytes_a_rem_less_than_len_less_than_b_rem() {
    struct TestStruct {
        a: crate::BytesMut,
        b: crate::BytesMut,
    }

    let mut test_struct = TestStruct {
        a: crate::BytesMut::from("123"),
        b: crate::BytesMut::from("abcdefgh"),
    };

    let result = test_struct.copy_to_bytes(5);
    assert_eq!(result.len(), 5);
    assert_eq!(&*result, b"123ab");
}

#[should_panic(expected = "`len` greater than remaining")]
#[test]
fn test_copy_to_bytes_a_rem_less_than_len_equal_to_b_rem() {
    struct TestStruct {
        a: crate::BytesMut,
        b: crate::BytesMut,
    }

    let mut test_struct = TestStruct {
        a: crate::BytesMut::from("123"),
        b: crate::BytesMut::from("ab"),
    };

    // This should panic because len exceeds total remaining
    let _ = test_struct.copy_to_bytes(6);
}

