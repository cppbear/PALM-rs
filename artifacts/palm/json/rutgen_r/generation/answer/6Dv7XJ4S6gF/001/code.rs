// Answer 0

#[derive(Default)]
struct Delegate {
    data: Vec<u8>,
    index: usize,
}

impl Delegate {
    fn peek(&mut self) -> Result<Option<u8>, String> {
        if self.index >= self.data.len() {
            return Ok(None);
        }
        Ok(Some(self.data[self.index]))
    }
}

struct Peeker<'a> {
    delegate: &'a mut Delegate,
}

impl<'a> Peeker<'a> {
    fn peek(&mut self) -> Result<Option<u8>, String> {
        self.delegate.peek()
    }
}

#[test]
fn test_peek_empty() {
    let mut delegate = Delegate { data: Vec::new(), index: 0 };
    let mut peeker = Peeker { delegate: &mut delegate };
    assert_eq!(peeker.peek().unwrap(), None);
}

#[test]
fn test_peek_one_element() {
    let mut delegate = Delegate { data: vec![42], index: 0 };
    let mut peeker = Peeker { delegate: &mut delegate };
    assert_eq!(peeker.peek().unwrap(), Some(42));
}

#[test]
fn test_peek_multiple_elements() {
    let mut delegate = Delegate { data: vec![1, 2, 3], index: 0 };
    let mut peeker = Peeker { delegate: &mut delegate };
    assert_eq!(peeker.peek().unwrap(), Some(1));
}

#[test]
fn test_peek_index_out_of_bounds() {
    let mut delegate = Delegate { data: vec![1, 2, 3], index: 3 };
    let mut peeker = Peeker { delegate: &mut delegate };
    assert_eq!(peeker.peek().unwrap(), None);
}

