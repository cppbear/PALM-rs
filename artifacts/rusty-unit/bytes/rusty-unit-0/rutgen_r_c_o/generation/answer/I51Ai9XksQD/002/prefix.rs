// Answer 0

#[test]
fn test_chunk_when_a_has_no_remaining() {
    struct TestBufA {
        data: Vec<u8>,
        position: usize,
    }

    impl Buf for TestBufA {
        fn remaining(&self) -> usize {
            self.data.len() - self.position
        }
        
        fn chunk(&self) -> &[u8] {
            &self.data[self.position..]
        }

        fn advance(&mut self, cnt: usize) {
            self.position += cnt;
        }
        
        fn has_remaining(&self) -> bool {
            self.remaining() > 0
        }

        fn copy_to_slice(&mut self, _dst: &mut [u8]) {}
        
        fn get_u8(&mut self) -> u8 {
            0
        }
    }
    
    struct TestBufB {
        data: Vec<u8>,
        position: usize,
    }

    impl Buf for TestBufB {
        fn remaining(&self) -> usize {
            self.data.len() - self.position
        }
        
        fn chunk(&self) -> &[u8] {
            &self.data[self.position..]
        }

        fn advance(&mut self, _cnt: usize) {}

        fn has_remaining(&self) -> bool {
            self.remaining() > 0
        }

        fn copy_to_slice(&mut self, _dst: &mut [u8]) {}
        
        fn get_u8(&mut self) -> u8 {
            0
        }
    }

    let buf_a = TestBufA { data: vec![], position: 0 };
    let buf_b = TestBufB { data: vec![1, 2, 3, 4, 5], position: 0 };
    let chain = Chain { a: buf_a, b: buf_b };

    let result = chain.chunk();
}

#[test]
fn test_chunk_when_a_has_no_remaining_and_b_has_remaining() {
    struct TestBufA {
        data: Vec<u8>,
        position: usize,
    }

    impl Buf for TestBufA {
        fn remaining(&self) -> usize {
            self.data.len() - self.position
        }
        
        fn chunk(&self) -> &[u8] {
            &self.data[self.position..]
        }

        fn advance(&mut self, cnt: usize) {
            self.position += cnt;
        }
        
        fn has_remaining(&self) -> bool {
            self.remaining() > 0
        }

        fn copy_to_slice(&mut self, _dst: &mut [u8]) {}
        
        fn get_u8(&mut self) -> u8 {
            0
        }
    }
    
    struct TestBufB {
        data: Vec<u8>,
        position: usize,
    }

    impl Buf for TestBufB {
        fn remaining(&self) -> usize {
            self.data.len() - self.position
        }
        
        fn chunk(&self) -> &[u8] {
            &self.data[self.position..]
        }

        fn advance(&mut self, _cnt: usize) {}

        fn has_remaining(&self) -> bool {
            self.remaining() > 0
        }

        fn copy_to_slice(&mut self, _dst: &mut [u8]) {}
        
        fn get_u8(&mut self) -> u8 {
            0
        }
    }

    let buf_a = TestBufA { data: vec![], position: 0 };
    let buf_b = TestBufB { data: vec![10, 20, 30, 40, 50], position: 0 };
    let chain = Chain { a: buf_a, b: buf_b };

    let result = chain.chunk();
}

