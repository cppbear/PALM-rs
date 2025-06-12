// Answer 0

#[test]
fn test_fmt_not_enough_bytes_available() {
    struct Bytes {
        requested: usize,
        available: usize,
    }

    impl core::fmt::Display for Bytes {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            write!(
                f,
                "Not enough bytes remaining in buffer to read value (requested {} but only {} available)",
                self.requested,
                self.available
            )
        }
    }

    let buffer_underflow = Bytes {
        requested: 10,
        available: 5,
    };
    let mut output = String::new();
    let result = write!(&mut output, "{}", buffer_underflow);
    assert!(result.is_ok());
    assert_eq!(
        output,
        "Not enough bytes remaining in buffer to read value (requested 10 but only 5 available)"
    );
}

#[test]
fn test_fmt_exact_bytes_available() {
    struct Bytes {
        requested: usize,
        available: usize,
    }

    impl core::fmt::Display for Bytes {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            write!(
                f,
                "Not enough bytes remaining in buffer to read value (requested {} but only {} available)",
                self.requested,
                self.available
            )
        }
    }

    let buffer_exact = Bytes {
        requested: 10,
        available: 10,
    };
    let mut output = String::new();
    let result = write!(&mut output, "{}", buffer_exact);
    assert!(result.is_ok());
    assert_eq!(
        output,
        "Not enough bytes remaining in buffer to read value (requested 10 but only 10 available)"
    );
}

#[test]
fn test_zero_bytes_requested() {
    struct Bytes {
        requested: usize,
        available: usize,
    }

    impl core::fmt::Display for Bytes {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            write!(
                f,
                "Not enough bytes remaining in buffer to read value (requested {} but only {} available)",
                self.requested,
                self.available
            )
        }
    }

    let buffer_zero_requested = Bytes {
        requested: 0,
        available: 5,
    };
    let mut output = String::new();
    let result = write!(&mut output, "{}", buffer_zero_requested);
    assert!(result.is_ok());
    assert_eq!(
        output,
        "Not enough bytes remaining in buffer to read value (requested 0 but only 5 available)"
    );
}

#[test]
fn test_no_bytes_available() {
    struct Bytes {
        requested: usize,
        available: usize,
    }

    impl core::fmt::Display for Bytes {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            write!(
                f,
                "Not enough bytes remaining in buffer to read value (requested {} but only {} available)",
                self.requested,
                self.available
            )
        }
    }

    let buffer_no_available = Bytes {
        requested: 10,
        available: 0,
    };
    let mut output = String::new();
    let result = write!(&mut output, "{}", buffer_no_available);
    assert!(result.is_ok());
    assert_eq!(
        output,
        "Not enough bytes remaining in buffer to read value (requested 10 but only 0 available)"
    );
}

