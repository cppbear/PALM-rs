// Answer 0

#[test]
fn test_serialize_u8_zero() {
    let writer = TestWriter::new();
    let mut serializer = Serializer {
        writer,
        formatter: TestFormatter::new(),
    };
    serializer.serialize_u8(0);
}

#[test]
fn test_serialize_u8_one() {
    let writer = TestWriter::new();
    let mut serializer = Serializer {
        writer,
        formatter: TestFormatter::new(),
    };
    serializer.serialize_u8(1);
}

#[test]
fn test_serialize_u8_max() {
    let writer = TestWriter::new();
    let mut serializer = Serializer {
        writer,
        formatter: TestFormatter::new(),
    };
    serializer.serialize_u8(255);
}

#[test]
fn test_serialize_u8_mid() {
    let writer = TestWriter::new();
    let mut serializer = Serializer {
        writer,
        formatter: TestFormatter::new(),
    };
    serializer.serialize_u8(128);
}

#[test]
fn test_serialize_u8_near_max() {
    let writer = TestWriter::new();
    let mut serializer = Serializer {
        writer,
        formatter: TestFormatter::new(),
    };
    serializer.serialize_u8(254);
}

#[test]
fn test_serialize_u8_near_zero() {
    let writer = TestWriter::new();
    let mut serializer = Serializer {
        writer,
        formatter: TestFormatter::new(),
    };
    serializer.serialize_u8(2);
}

struct TestWriter {
    // implementation of a test writer
}

impl TestWriter {
    fn new() -> Self {
        // initialize writer
    }
}

struct TestFormatter {
    // implementation of a test formatter
}

impl TestFormatter {
    fn new() -> Self {
        // initialize formatter
    }
}

