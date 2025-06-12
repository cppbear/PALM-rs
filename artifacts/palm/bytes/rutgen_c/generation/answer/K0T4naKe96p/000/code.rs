// Answer 0

#[test]
fn test_chain_new() {
    struct TestA;
    struct TestB;

    let a = TestA;
    let b = TestB;

    let chain = Chain::new(a, b);
    assert_eq!(std::mem::size_of_val(&chain.a), std::mem::size_of::<TestA>());
    assert_eq!(std::mem::size_of_val(&chain.b), std::mem::size_of::<TestB>());
}

#[test]
fn test_chain_first_ref() {
    struct TestA(i32);
    struct TestB;

    let a = TestA(5);
    let b = TestB;

    let chain = Chain::new(a, b);
    assert_eq!(chain.first_ref().0, 5);
}

#[test]
fn test_chain_last_ref() {
    struct TestA;
    struct TestB(i32);

    let a = TestA;
    let b = TestB(10);

    let chain = Chain::new(a, b);
    assert_eq!(chain.last_ref().0, 10);
}

#[test]
fn test_chain_first_mut() {
    struct TestA(i32);
    struct TestB;

    let mut a = TestA(15);
    let b = TestB;

    let mut chain = Chain::new(a, b);
    *chain.first_mut() = TestA(20);
    assert_eq!(chain.first_ref().0, 20);
}

#[test]
fn test_chain_last_mut() {
    struct TestA;
    struct TestB(i32);

    let a = TestA;
    let mut b = TestB(25);

    let mut chain = Chain::new(a, b);
    *chain.last_mut() = TestB(30);
    assert_eq!(chain.last_ref().0, 30);
}

#[test]
fn test_chain_into_inner() {
    struct TestA(i32);
    struct TestB(i32);

    let a = TestA(1);
    let b = TestB(2);
    
    let chain = Chain::new(a, b);
    let (a_inner, b_inner) = chain.into_inner();
    assert_eq!(a_inner.0, 1);
    assert_eq!(b_inner.0, 2);
}

