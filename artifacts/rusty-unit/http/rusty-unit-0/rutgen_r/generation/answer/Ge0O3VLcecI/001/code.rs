// Answer 0


use std::fmt;

struct Port {
    port: u16,
}

impl fmt::Debug for Port {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Port").field(&self.port).finish()
    }
}

#[test]
fn test_fmt_with_valid_port() {
    let port = Port { port: 8080 };
    let result = format!("{:?}", port);
    assert_eq!(result, "Port(8080)");
}

#[test]
fn test_fmt_with_minimum_port() {
    let port = Port { port: 1 };
    let result = format!("{:?}", port);
    assert_eq!(result, "Port(1)");
}

#[test]
fn test_fmt_with_maximum_port() {
    let port = Port { port: 65535 };
    let result = format!("{:?}", port);
    assert_eq!(result, "Port(65535)");
}


