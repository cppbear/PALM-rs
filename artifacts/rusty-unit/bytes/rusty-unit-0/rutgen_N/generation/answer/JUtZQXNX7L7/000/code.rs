// Answer 0

#[test]
fn test_fill_buf() {
    use std::io;
    use bytes::Buf;

    struct MockBuf {
        data: Vec<u8>,
    }

    impl MockBuf {
        fn new(data: Vec<u8>) -> Self {
            MockBuf { data }
        }
    }

    impl Buf for MockBuf {
        fn chunk(&self) -> &[u8] {
            &self.data
        }
        
        fn remaining(&self) -> usize {
            self.data.len()
        }
        
        fn advance(&mut self, cnt: usize) {
            self.data.drain(0..cnt);
        }
    }

    struct MockReader {
        buf: MockBuf,
    }

    impl MockReader {
        fn new(buf: MockBuf) -> Self {
            MockReader { buf }
        }

        fn fill_buf(&mut self) -> io::Result<&[u8]> {
            Ok(self.buf.chunk())
        }
    }

    let data = vec![1, 2, 3, 4, 5];
    let mut reader = MockReader::new(MockBuf::new(data));
    
    let result = reader.fill_buf().expect("fill_buf should not fail");
    assert_eq!(result, [1, 2, 3, 4, 5]);
}

