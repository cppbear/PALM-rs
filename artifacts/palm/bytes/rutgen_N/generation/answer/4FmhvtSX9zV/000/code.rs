// Answer 0

#[test]
fn test_advance() {
    struct Buffer {
        position: usize,
        data: Vec<u8>,
    }

    impl Buffer {
        fn advance(&mut self, cnt: usize) {
            self.position = (self.position + cnt).min(self.data.len());
        }
    }

    let mut buf = Buffer {
        position: 0,
        data: vec![1, 2, 3, 4, 5],
    };

    buf.advance(2);
    assert_eq!(buf.position, 2);

    buf.advance(3);
    assert_eq!(buf.position, 5); // Should not exceed the length of the data

    buf.advance(1);
    assert_eq!(buf.position, 5); // Should remain at the end of the buffer
}

