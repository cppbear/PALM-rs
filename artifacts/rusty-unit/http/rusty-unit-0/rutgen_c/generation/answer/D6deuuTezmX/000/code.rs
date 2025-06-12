// Answer 0

#[test]
fn test_port_as_u16() {
    struct DummyRepr;

    let port_instance = Port {
        port: 80,
        repr: DummyRepr,
    };

    assert_eq!(port_instance.as_u16(), 80);
}

#[test]
fn test_port_as_u16_boundary() {
    struct DummyRepr;

    let port_instance = Port {
        port: 0,
        repr: DummyRepr,
    };

    assert_eq!(port_instance.as_u16(), 0);

    let port_instance_max = Port {
        port: 65535,
        repr: DummyRepr,
    };

    assert_eq!(port_instance_max.as_u16(), 65535);
}

