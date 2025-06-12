// Answer 0

#[test]
fn test_port_debug() {
    struct TestRepr;

    let port = 8080;
    let uri_port = Port { port, repr: TestRepr };
    
    let result = format!("{:?}", uri_port);
    
    assert!(result.contains("Port("));
    assert!(result.contains(&port.to_string()));
}

#[test]
fn test_port_debug_with_boundary_value() {
    struct TestRepr;

    let port = 0;
    let uri_port = Port { port, repr: TestRepr };

    let result = format!("{:?}", uri_port);

    assert!(result.contains("Port("));
    assert!(result.contains(&port.to_string()));
}

