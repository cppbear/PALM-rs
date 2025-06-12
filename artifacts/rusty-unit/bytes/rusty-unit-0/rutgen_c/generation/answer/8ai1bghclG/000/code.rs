// Answer 0

#[test]
fn test_into_inner() {
    struct TestBuf;
    
    impl Buf for TestBuf {
        // Implement necessary Buf methods here for testing
    }

    struct TestBufMut;
    
    impl BufMut for TestBufMut {
        // Implement necessary BufMut methods here for testing
    }

    let first_buf = TestBuf;
    let last_buf = TestBufMut;

    let chain = Chain::new(first_buf, last_buf);
    
    let (first, last) = chain.into_inner();
    
    // Validate that first and last are of the expected types
    // For the sake of this example, we're not specifically checking content
    assert_eq!(std::any::type_name::<_>(&first), std::any::type_name::<TestBuf>());
    assert_eq!(std::any::type_name::<_>(&last), std::any::type_name::<TestBufMut>());
}

