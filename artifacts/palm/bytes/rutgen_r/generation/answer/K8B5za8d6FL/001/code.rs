// Answer 0

#[derive(Debug)]
struct Buf {
    data: Vec<u8>,
    pos: usize,
}

impl Buf {
    fn new(data: Vec<u8>, pos: usize) -> Self {
        Buf { data, pos }
    }

    fn get_ref(&self) -> &Vec<u8> {
        &self.data
    }

    fn position(&self) -> usize {
        self.pos
    }

    fn chunk(&self) -> &[u8] {
        let slice = self.get_ref().as_ref();
        let pos = min_u64_usize(self.position(), slice.len());
        &slice[pos..]
    }
}

fn min_u64_usize(value: usize, max: usize) -> usize {
    if value < max { value } else { max }
}

#[test]
fn test_chunk_position_zero() {
    let buf = Buf::new(vec![1, 2, 3, 4, 5], 0);
    let result = buf.chunk();
    assert_eq!(result, &[1, 2, 3, 4, 5]);
}

#[test]
fn test_chunk_position_within_bounds() {
    let buf = Buf::new(vec![1, 2, 3, 4, 5], 2);
    let result = buf.chunk();
    assert_eq!(result, &[3, 4, 5]);
}

#[test]
fn test_chunk_position_equal_to_length() {
    let buf = Buf::new(vec![1, 2, 3, 4, 5], 5);
    let result = buf.chunk();
    assert_eq!(result, &[]);
}

#[test]
fn test_chunk_position_exceeds_length() {
    let buf = Buf::new(vec![1, 2, 3, 4, 5], 10);
    let result = buf.chunk();
    assert_eq!(result, &[]);
}

#[test]
#[should_panic]
fn test_chunk_negative_position() {
    let buf = Buf::new(vec![1, 2, 3, 4, 5], usize::MAX);
    let _result = buf.chunk(); // This may panic
}

