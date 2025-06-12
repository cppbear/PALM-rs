// Answer 0

#[derive(Debug)]
struct Buffer {
    data: Vec<u8>,
    position: usize,
}

impl Buffer {
    fn new(data: Vec<u8>) -> Self {
        Self { data, position: 0 }
    }

    fn advance(&mut self, amt: usize) {
        self.position += amt;
    }
}

struct Reader {
    buf: Buffer,
}

impl Reader {
    fn new(data: Vec<u8>) -> Self {
        Self { buf: Buffer::new(data) }
    }

    fn consume(&mut self, amt: usize) {
        self.buf.advance(amt);
    }
}

#[test]
fn test_consume_valid_amount() {
    let mut reader = Reader::new(vec![1, 2, 3, 4, 5]);
    reader.consume(2);
    assert_eq!(reader.buf.position, 2);
}

#[test]
fn test_consume_exceeding_amount() {
    let mut reader = Reader::new(vec![1, 2, 3, 4, 5]);
    reader.consume(5);
    assert_eq!(reader.buf.position, 5);
}

#[test]
fn test_consume_zero_amount() {
    let mut reader = Reader::new(vec![1, 2, 3, 4, 5]);
    reader.consume(0);
    assert_eq!(reader.buf.position, 0);
}

#[should_panic]
#[test]
fn test_consume_negative_amount() {
    let mut reader = Reader::new(vec![1, 2, 3, 4, 5]);
    reader.consume(usize::MAX);
}

