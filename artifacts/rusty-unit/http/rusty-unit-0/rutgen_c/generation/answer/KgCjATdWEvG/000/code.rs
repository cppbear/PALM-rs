// Answer 0

#[test]
fn test_port_display() {
    struct TestRepr;

    let port = Port {
        port: 8080,
        repr: TestRepr,
    };

    let mut buffer = String::new();
    let result = write!(&mut buffer, "{}", port);
    
    assert!(result.is_ok());
    assert_eq!(buffer, "8080");
}

#[test]
fn test_port_zero_display() {
    struct TestRepr;

    let port = Port {
        port: 0,
        repr: TestRepr,
    };

    let mut buffer = String::new();
    let result = write!(&mut buffer, "{}", port);
    
    assert!(result.is_ok());
    assert_eq!(buffer, "0");
}

#[test]
fn test_port_max_display() {
    struct TestRepr;

    let port = Port {
        port: 65535,
        repr: TestRepr,
    };

    let mut buffer = String::new();
    let result = write!(&mut buffer, "{}", port);
    
    assert!(result.is_ok());
    assert_eq!(buffer, "65535");
}

