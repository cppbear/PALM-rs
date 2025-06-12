// Answer 0

#[derive(Debug)]
struct MockFormatter;

impl MockFormatter {
    fn end_object(&self, _writer: &mut dyn std::io::Write) -> Result<(), std::io::Error> {
        Ok(())
    }
}

struct MockWriter;

impl std::io::Write for MockWriter {
    fn write(&mut self, _buf: &[u8]) -> Result<usize, std::io::Error> {
        Ok(_buf.len())
    }

    fn flush(&mut self) -> Result<(), std::io::Error> {
        Ok(())
    }
}

#[derive(Debug)]
enum State {
    Empty,
    NonEmpty,
}

enum Compound {
    Map { ser: Serializer, state: State },
}

struct Serializer {
    formatter: MockFormatter,
    writer: MockWriter,
}

#[test]
fn test_end_with_non_empty_state() {
    let writer = MockWriter;
    let formatter = MockFormatter;
    let ser = Serializer { formatter, writer };
    let compound = Compound::Map { ser, state: State::NonEmpty };

    let result = match compound {
        Compound::Map { ser, state } => match state {
            State::Empty => Ok(()),
            _ => ser.formatter.end_object(&mut ser.writer).map_err(|e| e.into()),
        },
    };

    assert!(result.is_ok());
}

