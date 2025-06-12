// Answer 0

#[test]
fn test_end_with_map_state_empty() {
    struct TestWriter;
    impl io::Write for TestWriter {
        fn write(&mut self, _buf: &[u8]) -> Result<usize> {
            Ok(buf.len())
        }
        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    let mut serializer = Serializer {
        writer: TestWriter,
        formatter: CompactFormatter,
    };
    
    let mut compound = Compound::Map {
        ser: &mut serializer,
        state: State::Empty,
    };
    
    compound.end();
}

#[test]
fn test_end_with_map_state_first() {
    struct TestWriter;
    impl io::Write for TestWriter {
        fn write(&mut self, _buf: &[u8]) -> Result<usize> {
            Ok(buf.len())
        }
        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    let mut serializer = Serializer {
        writer: TestWriter,
        formatter: CompactFormatter,
    };
    
    let mut compound = Compound::Map {
        ser: &mut serializer,
        state: State::First,
    };
    
    compound.end();
}

#[test]
fn test_end_with_map_state_rest() {
    struct TestWriter;
    impl io::Write for TestWriter {
        fn write(&mut self, _buf: &[u8]) -> Result<usize> {
            Ok(buf.len())
        }
        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    let mut serializer = Serializer {
        writer: TestWriter,
        formatter: CompactFormatter,
    };
    
    let mut compound = Compound::Map {
        ser: &mut serializer,
        state: State::Rest,
    };
    
    compound.end();
}

#[test]
#[cfg(feature = "arbitrary_precision")]
fn test_end_with_number() {
    struct TestWriter;
    impl io::Write for TestWriter {
        fn write(&mut self, _buf: &[u8]) -> Result<usize> {
            Ok(buf.len())
        }
        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    let mut serializer = Serializer {
        writer: TestWriter,
        formatter: CompactFormatter,
    };
    
    let mut compound = Compound::Number {
        ser: &mut serializer,
    };
    
    compound.end();
}

#[test]
#[cfg(feature = "raw_value")]
fn test_end_with_raw_value() {
    struct TestWriter;
    impl io::Write for TestWriter {
        fn write(&mut self, _buf: &[u8]) -> Result<usize> {
            Ok(buf.len())
        }
        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    let mut serializer = Serializer {
        writer: TestWriter,
        formatter: CompactFormatter,
    };
    
    let mut compound = Compound::RawValue {
        ser: &mut serializer,
    };
    
    compound.end();
}

