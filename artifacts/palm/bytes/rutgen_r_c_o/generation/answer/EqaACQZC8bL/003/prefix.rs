// Answer 0

#[test]
fn test_advance_mut_a_rem_zero_and_cnt_zero() {
    struct BufMutImpl {
        buffer: Vec<u8>,
        position: usize,
    }

    impl BufMut for BufMutImpl {
        fn remaining_mut(&self) -> usize {
            0
        }
        
        fn chunk_mut(&mut self) -> &mut UninitSlice {
            unimplemented!()
        }

        unsafe fn advance_mut(&mut self, _: usize) {
            self.position += 0;
        }
    }
    
    let buf_a = BufMutImpl { buffer: vec![0; 10], position: 0 };
    let buf_b = BufMutImpl { buffer: vec![0; 10], position: 0 };
    let mut chain = Chain { a: buf_a, b: buf_b };

    unsafe {
        chain.advance_mut(0);
    }
}

#[test]
fn test_advance_mut_a_rem_zero_and_cnt_non_zero() {
    struct BufMutImpl {
        buffer: Vec<u8>,
        position: usize,
    }

    impl BufMut for BufMutImpl {
        fn remaining_mut(&self) -> usize {
            0
        }
        
        fn chunk_mut(&mut self) -> &mut UninitSlice {
            unimplemented!()
        }

        unsafe fn advance_mut(&mut self, _: usize) {
            self.position += 0;
        }
    }
    
    let buf_a = BufMutImpl { buffer: vec![0; 10], position: 0 };
    let buf_b = BufMutImpl { buffer: vec![0; 10], position: 0 };
    let mut chain = Chain { a: buf_a, b: buf_b };

    unsafe {
        chain.advance_mut(5);
    }
}

#[test]
fn test_advance_mut_a_rem_zero_and_cnt_equals_max() {
    const MAX_COUNT: usize = 10;
    
    struct BufMutImpl {
        buffer: Vec<u8>,
        position: usize,
    }

    impl BufMut for BufMutImpl {
        fn remaining_mut(&self) -> usize {
            0
        }
        
        fn chunk_mut(&mut self) -> &mut UninitSlice {
            unimplemented!()
        }

        unsafe fn advance_mut(&mut self, _: usize) {
            self.position += 0;
        }
    }
    
    let buf_a = BufMutImpl { buffer: vec![0; MAX_COUNT], position: 0 };
    let buf_b = BufMutImpl { buffer: vec![0; MAX_COUNT], position: 0 };
    let mut chain = Chain { a: buf_a, b: buf_b };

    unsafe {
        chain.advance_mut(MAX_COUNT);
    }
}

