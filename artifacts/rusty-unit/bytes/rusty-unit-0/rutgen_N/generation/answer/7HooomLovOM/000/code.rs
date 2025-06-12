// Answer 0

#[derive(Debug)]
struct BufMut {
    data: Vec<u8>,
}

impl BufMut {
    fn new() -> Self {
        BufMut { data: Vec::new() }
    }
    
    fn len(&self) -> usize {
        self.data.len()
    }
    
    fn resize(&mut self, new_len: usize, val: u8) {
        if new_len > self.len() {
            self.data.resize(new_len, val);
        } else {
            self.data.truncate(new_len);
        }
    }
    
    fn put_bytes(&mut self, val: u8, cnt: usize) {
        let new_len = self.len().saturating_add(cnt);
        self.resize(new_len, val);
    }
}

#[test]
fn test_put_bytes_increase_size() {
    let mut buf = BufMut::new();
    buf.put_bytes(1, 5);
    assert_eq!(buf.len(), 5);
    assert_eq!(buf.data, vec![1, 1, 1, 1, 1]);
}

#[test]
fn test_put_bytes_zero_count() {
    let mut buf = BufMut::new();
    buf.put_bytes(2, 0);
    assert_eq!(buf.len(), 0);
    assert_eq!(buf.data, Vec::<u8>::new());
}

#[test]
fn test_put_bytes_saturating_add() {
    let mut buf = BufMut::new();
    buf.put_bytes(3, usize::MAX);
    assert_eq!(buf.len(), usize::MAX);
    assert!(buf.data.iter().all(|&x| x == 3));
}

#[test]
fn test_put_bytes_decrease_size() {
    let mut buf = BufMut::new();
    buf.put_bytes(4, 10);
    buf.resize(5, 0); // Truncates the buffer
    buf.put_bytes(5, 2);
    assert_eq!(buf.len(), 7);
    assert_eq!(buf.data, vec![4, 4, 4, 4, 4, 5, 5]);
}

