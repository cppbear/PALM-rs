// Answer 0

fn saturating_sub_usize_u64(a: usize, b: u64) -> usize {
    if b > a as u64 {
        0
    } else {
        a - b as usize
    }
}

struct TestBuf {
    data: Vec<u8>,
    position: u64,
}

impl TestBuf {
    fn new(data: Vec<u8>) -> Self {
        TestBuf { data, position: 0 }
    }

    fn get_ref(&self) -> &Vec<u8> {
        &self.data
    }

    fn position(&self) -> u64 {
        self.position
    }

    fn set_position(&mut self, pos: u64) {
        self.position = pos;
    }

    fn advance(&mut self, cnt: usize) {
        let len = self.get_ref().as_ref().len();
        let pos = self.position();

        let max_cnt = saturating_sub_usize_u64(len, pos);
        if cnt > max_cnt {
            panic!("Advance error: requested {} available {}", cnt, max_cnt);
        }

        self.set_position(pos + cnt as u64);
    }
}

#[test]
#[should_panic(expected = "Advance error: requested 10 available 5")]
fn test_advance_panic_condition() {
    let mut buf = TestBuf::new(vec![1, 2, 3, 4, 5]);
    buf.set_position(5); // Set position to equal the length of the data
    buf.advance(10); // This should trigger the panic condition
}

#[test]
fn test_advance_no_panic() {
    let mut buf = TestBuf::new(vec![1, 2, 3, 4, 5]);
    buf.advance(0); // Valid case, should not panic
    assert_eq!(buf.position(), 0);

    buf.set_position(3);
    buf.advance(2); // Advance 2 from position 3, valid
    assert_eq!(buf.position(), 5); // Position should now be 5
}

