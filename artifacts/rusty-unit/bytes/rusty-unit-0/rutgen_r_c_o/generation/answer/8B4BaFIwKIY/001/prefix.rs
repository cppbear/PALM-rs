// Answer 0

#[test]
fn test_get_mut_with_u32_min() {
    struct TestBuf {
        data: u32,
    }
    
    impl Buf for TestBuf {}

    let mut buf = TestBuf { data: u32::MIN };
    let mut iter = IntoIter::new(buf);
    let mut_ref = iter.get_mut();
}

#[test]
fn test_get_mut_with_u32_max() {
    struct TestBuf {
        data: u32,
    }

    impl Buf for TestBuf {}

    let mut buf = TestBuf { data: u32::MAX };
    let mut iter = IntoIter::new(buf);
    let mut_ref = iter.get_mut();
}

#[test]
fn test_get_mut_with_mid_value() {
    struct TestBuf {
        data: u32,
    }

    impl Buf for TestBuf {}

    let mut buf = TestBuf { data: 500000000 };
    let mut iter = IntoIter::new(buf);
    let mut_ref = iter.get_mut();
}

#[test]
#[should_panic]
fn test_get_mut_on_empty_struct() {
    struct EmptyBuf {}

    impl Buf for EmptyBuf {}

    let mut buf = EmptyBuf {};
    let mut iter = IntoIter::new(buf);
    let mut_ref = iter.get_mut();
}

