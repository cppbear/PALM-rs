// Answer 0

#[test]
fn test_fmt_with_enough_bytes() {
    struct Buffer {
        requested: usize,
        available: usize,
    }

    impl core::fmt::Debug for Buffer {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            write!(
                f,
                "Not enough bytes remaining in buffer to read value (requested {} but only {} available)",
                self.requested,
                self.available
            )
        }
    }

    let buffer = Buffer {
        requested: 10,
        available: 20,
    };
    
    let mut output = String::new();
    let result = format_args!("{:?}", buffer);
    write!(&mut output, "{}", result).unwrap();

    assert_eq!(
        output,
        "Not enough bytes remaining in buffer to read value (requested 10 but only 20 available)"
    );
}

#[test]
fn test_fmt_with_not_enough_bytes() {
    struct Buffer {
        requested: usize,
        available: usize,
    }

    impl core::fmt::Debug for Buffer {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            write!(
                f,
                "Not enough bytes remaining in buffer to read value (requested {} but only {} available)",
                self.requested,
                self.available
            )
        }
    }

    let buffer = Buffer {
        requested: 20,
        available: 10,
    };
    
    let mut output = String::new();
    let result = format_args!("{:?}", buffer);
    write!(&mut output, "{}", result).unwrap();

    assert_eq!(
        output,
        "Not enough bytes remaining in buffer to read value (requested 20 but only 10 available)"
    );
}

