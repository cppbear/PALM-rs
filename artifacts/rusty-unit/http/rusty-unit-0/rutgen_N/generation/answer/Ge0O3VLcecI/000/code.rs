// Answer 0

#[test]
fn test_fmt() {
    use std::fmt;

    struct Port {
        port: u16,
    }

    impl Port {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_tuple("Port").field(&self.port).finish()
        }
    }

    let port_instance = Port { port: 80 };
    
    let mut output = vec![];
    {
        let formatter = &mut fmt::Formatter::new(&mut output);
        port_instance.fmt(formatter).unwrap();
    }

    let result = String::from_utf8(output).unwrap();
    assert_eq!(result, "Port(80)");
}

#[test]
fn test_fmt_with_boundary_port() {
    use std::fmt;

    struct Port {
        port: u16,
    }

    impl Port {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_tuple("Port").field(&self.port).finish()
        }
    }

    let port_instance = Port { port: 65535 };
    
    let mut output = vec![];
    {
        let formatter = &mut fmt::Formatter::new(&mut output);
        port_instance.fmt(formatter).unwrap();
    }

    let result = String::from_utf8(output).unwrap();
    assert_eq!(result, "Port(65535)");
}

#[test]
fn test_fmt_with_zero_port() {
    use std::fmt;

    struct Port {
        port: u16,
    }

    impl Port {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_tuple("Port").field(&self.port).finish()
        }
    }

    let port_instance = Port { port: 0 };
    
    let mut output = vec![];
    {
        let formatter = &mut fmt::Formatter::new(&mut output);
        port_instance.fmt(formatter).unwrap();
    }

    let result = String::from_utf8(output).unwrap();
    assert_eq!(result, "Port(0)");
}

