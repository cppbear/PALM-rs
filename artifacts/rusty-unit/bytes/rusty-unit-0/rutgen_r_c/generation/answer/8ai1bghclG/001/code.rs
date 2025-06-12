// Answer 0

#[test]
fn test_into_inner_with_basic_types() {
    struct A;
    struct B;

    let chain = Chain::new(A, B);
    let (first, last) = chain.into_inner();
    assert_eq!(std::mem::discriminant(&first), std::mem::discriminant(&A));
    assert_eq!(std::mem::discriminant(&last), std::mem::discriminant(&B));
}

#[test]
fn test_into_inner_with_slice_types() {
    let first_slice: &[u8] = b"first";
    let last_slice: &[u8] = b"last";
    
    let chain = Chain::new(first_slice, last_slice);
    let (first, last) = chain.into_inner();
    assert_eq!(first, first_slice);
    assert_eq!(last, last_slice);
}

#[test]
fn test_into_inner_with_empty_chain() {
    struct Empty;
    
    let chain = Chain::new(Empty, Empty);
    let (first, last) = chain.into_inner();
    assert_eq!(std::mem::discriminant(&first), std::mem::discriminant(&Empty));
    assert_eq!(std::mem::discriminant(&last), std::mem::discriminant(&Empty));
}

