// Answer 0

#[test]
fn test_reserve_case1() {
    let mut buf = BytesMut::with_capacity(10);
    buf.reserve(11); // additional exceeds current available capacity
}

#[test]
fn test_reserve_case2() {
    let mut buf = BytesMut::with_capacity(20);
    buf.reserve(21); // additional exceeds current available capacity 
}

#[test]
fn test_reserve_case3() {
    let mut buf = BytesMut::with_capacity(30);
    buf.resize(29, 0); // set len to 29
    buf.reserve(2); // additional exceeds current available capacity
}

#[test]
fn test_reserve_case4() {
    let mut buf = BytesMut::with_capacity(50);
    buf.resize(40, 0); // set len to 40
    buf.reserve(11); // additional exceeds current available capacity
}

#[test]
#[should_panic]
fn test_reserve_case5() {
    let mut buf = BytesMut::with_capacity(usize::MAX);
    buf.reserve(1); // should panic due to overflow when trying to add more capacity
}

#[test]
fn test_reserve_case6() {
    let mut buf = BytesMut::with_capacity(15);
    buf.resize(10, 0); // set len to 10
    buf.reserve(10); // additional exceeds current available capacity
}

