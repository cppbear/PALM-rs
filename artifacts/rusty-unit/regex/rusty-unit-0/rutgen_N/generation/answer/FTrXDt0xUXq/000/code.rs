// Answer 0

#[derive(Debug)]
struct Byte(Option<u8>);

impl Byte {
    fn as_byte(&self) -> Option<u8> {
        self.0
    }
}

struct DummyDFA {
    num_byte_classes: usize,
}

impl DummyDFA {
    fn num_byte_classes(&self) -> usize {
        self.num_byte_classes
    }

    fn u8_class(&self, b: u8) -> usize {
        b as usize // A simple implementation for the sake of the test.
    }
}

impl DummyDFA {
    fn byte_class(&self, b: Byte) -> usize {
        match b.as_byte() {
            None => self.num_byte_classes() - 1,
            Some(b) => self.u8_class(b),
        }
    }
}

#[test]
fn test_byte_class_with_valid_byte() {
    let dfa = DummyDFA { num_byte_classes: 256 };
    let byte = Byte(Some(5));
    assert_eq!(dfa.byte_class(byte), 5);
}

#[test]
fn test_byte_class_with_eof() {
    let dfa = DummyDFA { num_byte_classes: 256 };
    let byte = Byte(None);
    assert_eq!(dfa.byte_class(byte), 255);
}

#[test]
fn test_byte_class_with_edge_case() {
    let dfa = DummyDFA { num_byte_classes: 1 };
    let byte = Byte(None);
    assert_eq!(dfa.byte_class(byte), 0);
}

