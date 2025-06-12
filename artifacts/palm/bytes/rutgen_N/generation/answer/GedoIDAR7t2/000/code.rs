// Answer 0

#[derive(Debug)]
struct TestBuffer {
    position: u64,
    data: Vec<u8>,
}

impl TestBuffer {
    fn new(data: Vec<u8>) -> Self {
        TestBuffer { position: 0, data }
    }

    fn get_ref(&self) -> &[u8] {
        &self.data
    }

    fn position(&self) -> u64 {
        self.position
    }

    fn set_position(&mut self, pos: u64) {
        self.position = pos;
    }

    fn advance(&mut self, cnt: usize) {
        let len = self.get_ref().len();
        let pos = self.position();

        let max_cnt = len.saturating_sub(pos as usize);
        if cnt > max_cnt {
            panic!("Requested {} is greater than available {}", cnt, max_cnt);
        }

        self.set_position(pos + cnt as u64);
    }
}

#[test]
fn test_advance_with_zero() {
    let mut buf = TestBuffer::new(vec![1, 2, 3, 4, 5]);
    buf.advance(0);
    assert_eq!(buf.position(), 0);
}

#[test]
fn test_advance_within_bounds() {
    let mut buf = TestBuffer::new(vec![1, 2, 3, 4, 5]);
    buf.advance(3);
    assert_eq!(buf.position(), 3);
}

#[test]
#[should_panic(expected = "Requested 10 is greater than available 5")]
fn test_advance_out_of_bounds() {
    let mut buf = TestBuffer::new(vec![1, 2, 3, 4, 5]);
    buf.advance(10);
}

#[test]
fn test_advance_to_end() {
    let mut buf = TestBuffer::new(vec![1, 2, 3, 4, 5]);
    buf.advance(5);
    assert_eq!(buf.position(), 5);
}

#[test]
fn test_advance_more_than_current_position() {
    let mut buf = TestBuffer::new(vec![1, 2, 3, 4, 5]);
    buf.advance(2);
    assert_eq!(buf.position(), 2);
    buf.advance(3);
    assert_eq!(buf.position(), 5);
}

