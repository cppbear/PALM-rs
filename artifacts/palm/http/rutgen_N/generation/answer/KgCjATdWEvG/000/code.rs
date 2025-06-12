// Answer 0

#[derive(Debug)]
struct Port {
    port: u16,
}

impl std::fmt::Display for Port {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Use `u16::fmt` so that it respects any formatting flags that
        // may have been set (like padding, align, etc).
        std::fmt::Display::fmt(&self.port, f)
    }
}

#[test]
fn test_format_port_normal() {
    let port = Port { port: 8080 };
    let mut buffer = String::new();
    let result = write!(&mut buffer, "{}", port);
    assert!(result.is_ok());
    assert_eq!(buffer, "8080");
}

#[test]
fn test_format_port_zero() {
    let port = Port { port: 0 };
    let mut buffer = String::new();
    let result = write!(&mut buffer, "{}", port);
    assert!(result.is_ok());
    assert_eq!(buffer, "0");
}

#[test]
fn test_format_port_max() {
    let port = Port { port: 65535 };
    let mut buffer = String::new();
    let result = write!(&mut buffer, "{}", port);
    assert!(result.is_ok());
    assert_eq!(buffer, "65535");
}

