// Answer 0

#[test]
fn test_port_as_u16_basic() {
    struct TestRepr;
    let port = Port::<TestRepr> { port: 80, repr: TestRepr };
    assert_eq!(port.as_u16(), 80);
}

#[test]
fn test_port_as_u16_zero() {
    struct TestRepr;
    let port = Port::<TestRepr> { port: 0, repr: TestRepr };
    assert_eq!(port.as_u16(), 0);
}

#[test]
fn test_port_as_u16_max() {
    struct TestRepr;
    let port = Port::<TestRepr> { port: 65535, repr: TestRepr };
    assert_eq!(port.as_u16(), 65535);
}

#[test]
fn test_port_as_u16_edge() {
    struct TestRepr;
    let port_low = Port::<TestRepr> { port: 1, repr: TestRepr };
    let port_high = Port::<TestRepr> { port: 65534, repr: TestRepr };
    assert_eq!(port_low.as_u16(), 1);
    assert_eq!(port_high.as_u16(), 65534);
}

