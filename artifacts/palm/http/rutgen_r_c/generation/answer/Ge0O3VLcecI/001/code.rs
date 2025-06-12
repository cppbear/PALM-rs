// Answer 0

#[test]
fn test_port_debug_fmt() {
    struct TestRepr;

    impl fmt::Debug for TestRepr {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str("TestRepr")
        }
    }

    let port_instance = Port {
        port: 8080, // A common port number
        repr: TestRepr,
    };

    let mut output = String::new();
    let result = write!(&mut output, "{:?}", port_instance);
    assert!(result.is_ok()); // Ensure write was successful

    assert_eq!(output, "Port(8080)"); // Check that the output matches expected format
}

#[test]
fn test_port_debug_fmt_zero() {
    struct TestRepr;

    impl fmt::Debug for TestRepr {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str("TestRepr")
        }
    }

    let port_instance = Port {
        port: 0, // Edge case: port number at the minimum valid value
        repr: TestRepr,
    };

    let mut output = String::new();
    let result = write!(&mut output, "{:?}", port_instance);
    assert!(result.is_ok()); // Ensure write was successful

    assert_eq!(output, "Port(0)"); // Check output format for zero port
}

