// Answer 0

#[test]
fn test_chain_new() {
    struct Chain<T, U> {
        a: T,
        b: U,
    }

    fn new<T, U>(a: T, b: U) -> Chain<T, U> {
        Chain { a, b }
    }

    let chain = new(1, "test");
    assert_eq!(chain.a, 1);
    assert_eq!(chain.b, "test");

    let chain2 = new(vec![1, 2, 3], 3.14);
    assert_eq!(chain2.a, vec![1, 2, 3]);
    assert_eq!(chain2.b, 3.14);
}

