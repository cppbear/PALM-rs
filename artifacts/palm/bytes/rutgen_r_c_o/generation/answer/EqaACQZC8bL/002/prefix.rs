// Answer 0

#[test]
fn test_advance_mut_case_1() {
    struct BufMutImpl {
        buffer: Vec<u8>,
        position: usize,
    }
    
    unsafe impl BufMut for BufMutImpl {
        fn remaining_mut(&self) -> usize {
            self.buffer.len() - self.position
        }
        fn chunk_mut(&mut self) -> &mut UninitSlice {
            unimplemented!()
        }
        unsafe fn advance_mut(&mut self, cnt: usize) {
            self.position += cnt;
        }
    }
    
    let buf_a = BufMutImpl { buffer: vec![0; 5], position: 0 };
    let buf_b = BufMutImpl { buffer: vec![0; 10], position: 0 };
    
    let mut chain = Chain { a: buf_a, b: buf_b };
    let cnt = 6; // a_rem = 5, cnt = 6

    unsafe {
        chain.advance_mut(cnt);
    }
}

#[test]
fn test_advance_mut_case_2() {
    struct BufMutImpl {
        buffer: Vec<u8>,
        position: usize,
    }
    
    unsafe impl BufMut for BufMutImpl {
        fn remaining_mut(&self) -> usize {
            self.buffer.len() - self.position
        }
        fn chunk_mut(&mut self) -> &mut UninitSlice {
            unimplemented!()
        }
        unsafe fn advance_mut(&mut self, cnt: usize) {
            self.position += cnt;
        }
    }
    
    let buf_a = BufMutImpl { buffer: vec![0; 7], position: 0 };
    let buf_b = BufMutImpl { buffer: vec![0; 3], position: 0 };
    
    let mut chain = Chain { a: buf_a, b: buf_b };
    let cnt = 8; // a_rem = 7, cnt = 8

    unsafe {
        chain.advance_mut(cnt);
    }
}

#[test]
fn test_advance_mut_case_3() {
    struct BufMutImpl {
        buffer: Vec<u8>,
        position: usize,
    }
    
    unsafe impl BufMut for BufMutImpl {
        fn remaining_mut(&self) -> usize {
            self.buffer.len() - self.position
        }
        fn chunk_mut(&mut self) -> &mut UninitSlice {
            unimplemented!()
        }
        unsafe fn advance_mut(&mut self, cnt: usize) {
            self.position += cnt;
        }
    }

    let buf_a = BufMutImpl { buffer: vec![0; 15], position: 0 };
    let buf_b = BufMutImpl { buffer: vec![0; 5], position: 0 };

    let mut chain = Chain { a: buf_a, b: buf_b };
    let cnt = 16; // a_rem = 15, cnt = 16

    unsafe {
        chain.advance_mut(cnt);
    }
}

#[test]
fn test_advance_mut_case_4() {
    struct BufMutImpl {
        buffer: Vec<u8>,
        position: usize,
    }
    
    unsafe impl BufMut for BufMutImpl {
        fn remaining_mut(&self) -> usize {
            self.buffer.len() - self.position
        }
        fn chunk_mut(&mut self) -> &mut UninitSlice {
            unimplemented!()
        }
        unsafe fn advance_mut(&mut self, cnt: usize) {
            self.position += cnt;
        }
    }

    let buf_a = BufMutImpl { buffer: vec![0; 3], position: 0 };
    let buf_b = BufMutImpl { buffer: vec![0; 8], position: 0 };

    let mut chain = Chain { a: buf_a, b: buf_b };
    let cnt = 4; // a_rem = 3, cnt = 4

    unsafe {
        chain.advance_mut(cnt);
    }
}

