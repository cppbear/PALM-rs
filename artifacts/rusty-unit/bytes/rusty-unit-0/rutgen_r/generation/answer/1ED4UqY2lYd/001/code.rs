// Answer 0

#[test]
fn test_get_mut_non_empty_buf() {
    struct TestBuf {
        data: Vec<u8>,
    }
    
    struct TestReader {
        buf: TestBuf,
    }

    let mut reader = TestReader {
        buf: TestBuf { data: vec![1, 2, 3] },
    };
    
    let buf_ref: &mut TestBuf = reader.get_mut();
    buf_ref.data.push(4);
    
    assert_eq!(buf_ref.data, vec![1, 2, 3, 4]);
}

#[test]
fn test_get_mut_empty_buf() {
    struct TestBuf {
        data: Vec<u8>,
    }

    struct TestReader {
        buf: TestBuf,
    }

    let mut reader = TestReader {
        buf: TestBuf { data: vec![] },
    };
    
    let buf_ref: &mut TestBuf = reader.get_mut();
    buf_ref.data.push(1);
    
    assert_eq!(buf_ref.data, vec![1]);
}

#[test]
#[should_panic]
fn test_get_mut_double_borrow() {
    struct TestBuf {
        data: Vec<u8>,
    }

    struct TestReader {
        buf: TestBuf,
    }

    let mut reader = TestReader {
        buf: TestBuf { data: vec![1, 2, 3] },
    };
    
    let _buf_ref_1: &mut TestBuf = reader.get_mut();
    let _buf_ref_2: &mut TestBuf = reader.get_mut(); // This should panic
}

