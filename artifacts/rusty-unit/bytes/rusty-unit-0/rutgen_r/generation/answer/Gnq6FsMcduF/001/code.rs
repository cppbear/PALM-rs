// Answer 0


struct VecDeque {
    data: Vec<u8>,
}

impl VecDeque {
    fn new() -> Self {
        VecDeque { data: Vec::new() }
    }

    fn drain(&mut self, range: std::ops::Range<usize>) -> Vec<u8> {
        let range_end = range.end.min(self.data.len());
        let drained = self.data[range.start..range_end].to_vec();
        self.data.drain(range);
        drained
    }

    fn advance(&mut self, cnt: usize) {
        self.drain(..cnt);
    }
}

#[test]
fn test_advance_empty_vec_deque() {
    let mut vec_deque = VecDeque::new();
    vec_deque.advance(0);
    assert!(vec_deque.data.is_empty());
}

#[test]
fn test_advance_single_element() {
    let mut vec_deque = VecDeque::new();
    vec_deque.data.push(1);
    vec_deque.advance(1);
    assert!(vec_deque.data.is_empty());
}

#[test]
fn test_advance_greater_than_length() {
    let mut vec_deque = VecDeque::new();
    vec_deque.data.push(1);
    vec_deque.advance(5);
    assert!(vec_deque.data.is_empty());
}

#[test]
fn test_advance_exact_length() {
    let mut vec_deque = VecDeque::new();
    vec_deque.data.push(1);
    vec_deque.advance(1);
    assert!(vec_deque.data.is_empty());
}

#[test]
fn test_advance_multiple_elements() {
    let mut vec_deque = VecDeque::new();
    vec_deque.data.push(1);
    vec_deque.data.push(2);
    vec_deque.advance(1);
    assert_eq!(vec_deque.data.len(), 1);
    assert_eq!(vec_deque.data[0], 2);
}

#[test]
fn test_advance_to_zero() {
    let mut vec_deque = VecDeque::new();
    vec_deque.data.push(1);
    vec_deque.advance(0);
    assert_eq!(vec_deque.data.len(), 1);
    assert_eq!(vec_deque.data[0], 1);
}


