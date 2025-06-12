// Answer 0

#[test]
fn test_port_display() {
    struct TestPort {
        port: u16,
    }

    impl fmt::Display for TestPort {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            fmt::Display::fmt(&self.port, f)
        }
    }

    let p1 = TestPort { port: 80 };
    let p2 = TestPort { port: 443 };
    let p3 = TestPort { port: 65535 };
    let p4 = TestPort { port: 0 };

    let mut buf = String::new();
    
    // Test common ports
    assert_eq!(write!(&mut buf, "{}", p1).unwrap(), ());
    assert_eq!(buf, "80");
    buf.clear();

    assert_eq!(write!(&mut buf, "{}", p2).unwrap(), ());
    assert_eq!(buf, "443");
    buf.clear();

    // Test maximum valid port
    assert_eq!(write!(&mut buf, "{}", p3).unwrap(), ());
    assert_eq!(buf, "65535");
    buf.clear();

    // Test minimum valid port
    assert_eq!(write!(&mut buf, "{}", p4).unwrap(), ());
    assert_eq!(buf, "0");
    buf.clear();
}

