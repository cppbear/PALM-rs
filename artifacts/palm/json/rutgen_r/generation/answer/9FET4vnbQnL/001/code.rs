// Answer 0

#[test]
fn test_serialize_i32_success() {
    struct MockFormatter;

    impl MockFormatter {
        fn write_i32<W: std::io::Write>(&self, writer: &mut W, value: i32) -> std::io::Result<()> {
            write!(writer, "{}", value)?;
            Ok(())
        }
    }

    struct Serializer<W> {
        formatter: MockFormatter,
        writer: W,
    }

    let mut buf = Vec::new();
    let serializer = Serializer {
        formatter: MockFormatter,
        writer: &mut buf,
    };

    let result = serializer.serialize_i32(42);
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(buf).unwrap(), "42");
}

#[test]
fn test_serialize_i32_zero() {
    struct MockFormatter;

    impl MockFormatter {
        fn write_i32<W: std::io::Write>(&self, writer: &mut W, value: i32) -> std::io::Result<()> {
            write!(writer, "{}", value)?;
            Ok(())
        }
    }

    struct Serializer<W> {
        formatter: MockFormatter,
        writer: W,
    }

    let mut buf = Vec::new();
    let serializer = Serializer {
        formatter: MockFormatter,
        writer: &mut buf,
    };

    let result = serializer.serialize_i32(0);
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(buf).unwrap(), "0");
}

#[test]
fn test_serialize_i32_negative() {
    struct MockFormatter;

    impl MockFormatter {
        fn write_i32<W: std::io::Write>(&self, writer: &mut W, value: i32) -> std::io::Result<()> {
            write!(writer, "{}", value)?;
            Ok(())
        }
    }

    struct Serializer<W> {
        formatter: MockFormatter,
        writer: W,
    }

    let mut buf = Vec::new();
    let serializer = Serializer {
        formatter: MockFormatter,
        writer: &mut buf,
    };

    let result = serializer.serialize_i32(-42);
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(buf).unwrap(), "-42");
}

#[test]
fn test_serialize_i32_io_error() {
    struct MockFormatter;

    impl MockFormatter {
        fn write_i32<W: std::io::Write>(&self, _: &mut W, _: i32) -> std::io::Result<()> {
            Err(std::io::Error::new(std::io::ErrorKind::Other, "write error"))
        }
    }

    struct Serializer<W> {
        formatter: MockFormatter,
        writer: W,
    }

    let mut buf = Vec::new();
    let serializer = Serializer {
        formatter: MockFormatter,
        writer: &mut buf,
    };

    let result = serializer.serialize_i32(42);
    assert!(result.is_err());
}

