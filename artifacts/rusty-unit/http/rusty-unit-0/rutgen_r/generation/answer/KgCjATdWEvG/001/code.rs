// Answer 0

#[test]
fn test_fmt_with_valid_port() {
    struct Port {
        port: u16,
    }

    let port = Port { port: 8080 };
    let mut output = String::new();
    let result = write!(&mut output, "{}", port);
    assert!(result.is_ok());
    assert_eq!(output, "8080");
}

#[test]
fn test_fmt_with_minimum_port() {
    struct Port {
        port: u16,
    }

    let port = Port { port: 0 };
    let mut output = String::new();
    let result = write!(&mut output, "{}", port);
    assert!(result.is_ok());
    assert_eq!(output, "0");
}

#[test]
fn test_fmt_with_maximum_port() {
    struct Port {
        port: u16,
    }

    let port = Port { port: 65535 };
    let mut output = String::new();
    let result = write!(&mut output, "{}", port);
    assert!(result.is_ok());
    assert_eq!(output, "65535");
}

#[test]
fn test_fmt_with_zero_pad() {
    struct Port {
        port: u16,
    }

    let port = Port { port: 5 };
    let mut output = String::new();
    let result = write!(&mut output, "{:03}", port);
    assert!(result.is_ok());
    assert_eq!(output, "005");
}

