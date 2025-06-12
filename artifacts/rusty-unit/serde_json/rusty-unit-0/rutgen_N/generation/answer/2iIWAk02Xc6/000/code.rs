// Answer 0

#[derive(Debug)]
struct MockFormatter {
    array_started: bool,
    array_ended: bool,
}

impl MockFormatter {
    fn new() -> Self {
        Self {
            array_started: false,
            array_ended: false,
        }
    }
    
    fn begin_array(&mut self, _writer: &mut Vec<u8>) -> Result<(), std::io::Error> {
        self.array_started = true;
        Ok(())
    }
    
    fn end_array(&mut self, _writer: &mut Vec<u8>) -> Result<(), std::io::Error> {
        if !self.array_started {
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "Array not started"));
        }
        self.array_ended = true;
        Ok(())
    }
}

struct Serializer<'a> {
    formatter: MockFormatter,
    writer: &'a mut Vec<u8>,
}

impl<'a> Serializer<'a> {
    fn new(writer: &'a mut Vec<u8>) -> Self {
        Self {
            formatter: MockFormatter::new(),
            writer,
        }
    }
    
    fn serialize_seq(self, len: Option<usize>) -> Result<Compound<Serializer<'a>>, std::io::Error> {
        self.formatter.begin_array(self.writer)?;
        if len == Some(0) {
            self.formatter.end_array(self.writer)?;
            Ok(Compound::Map {
                ser: self,
                state: State::Empty,
            })
        } else {
            Ok(Compound::Map {
                ser: self,
                state: State::First,
            })
        }
    }
}

#[derive(Debug)]
enum Compound<S> {
    Map {
        ser: S,
        state: State,
    },
}

#[derive(Debug)]
enum State {
    Empty,
    First,
}

#[test]
fn test_serialize_seq_with_length_zero() {
    let mut writer = Vec::new();
    let serializer = Serializer::new(&mut writer);
    let result = serializer.serialize_seq(Some(0));

    assert!(result.is_ok());
    assert!(writer.is_empty());
}

#[test]
fn test_serialize_seq_with_length_non_zero() {
    let mut writer = Vec::new();
    let serializer = Serializer::new(&mut writer);
    let result = serializer.serialize_seq(Some(1));

    assert!(result.is_ok());
    assert!(writer.is_empty());
}

