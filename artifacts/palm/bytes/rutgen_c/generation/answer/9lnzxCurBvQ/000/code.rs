// Answer 0

#[test]
fn test_chunk_mut_a_has_remaining_mut() {
    struct BufMutA {
        data: Vec<u8>,
        position: usize,
    }
    
    struct BufMutB {
        data: Vec<u8>,
        position: usize,
    }

    unsafe impl BufMut for BufMutA {
        fn remaining_mut(&self) -> usize {
            self.data.len() - self.position
        }
        
        fn chunk_mut(&mut self) -> &mut UninitSlice {
            // Simulating chunk return, normally would be more complex
            let mut uninit_slice = UninitSlice(vec![MaybeUninit::new(0); 10]);
            &mut uninit_slice
        }
        
        unsafe fn advance_mut(&mut self, cnt: usize) {
            self.position += cnt;
        }
    }

    unsafe impl BufMut for BufMutB {
        fn remaining_mut(&self) -> usize {
            self.data.len() - self.position
        }
        
        fn chunk_mut(&mut self) -> &mut UninitSlice {
            let mut uninit_slice = UninitSlice(vec![MaybeUninit::new(0); 10]);
            &mut uninit_slice
        }
        
        unsafe fn advance_mut(&mut self, cnt: usize) {
            self.position += cnt;
        }
    }
    
    let mut a = BufMutA { data: vec![0; 20], position: 5 };
    let mut b = BufMutB { data: vec![0; 15], position: 0 };

    let mut chain = Chain { a, b };

    let chunk = chain.chunk_mut();
    assert!(chunk.0.len() > 0);
}

#[test]
fn test_chunk_mut_b_has_remaining_mut() {
    struct BufMutA {
        data: Vec<u8>,
        position: usize,
    }
    
    struct BufMutB {
        data: Vec<u8>,
        position: usize,
    }

    unsafe impl BufMut for BufMutA {
        fn remaining_mut(&self) -> usize {
            self.data.len() - self.position
        }
        
        fn chunk_mut(&mut self) -> &mut UninitSlice {
            let mut uninit_slice = UninitSlice(vec![MaybeUninit::new(0); 10]);
            &mut uninit_slice
        }
        
        unsafe fn advance_mut(&mut self, cnt: usize) {
            self.position += cnt;
        }
    }

    unsafe impl BufMut for BufMutB {
        fn remaining_mut(&self) -> usize {
            0  // No remaining for B, to test this branch
        }
        
        fn chunk_mut(&mut self) -> &mut UninitSlice {
            let mut uninit_slice = UninitSlice(vec![MaybeUninit::new(0); 10]);
            &mut uninit_slice
        }
        
        unsafe fn advance_mut(&mut self, cnt: usize) {
            self.position += cnt;
        }
    }
    
    let mut a = BufMutA { data: vec![0; 20], position: 20 };
    let mut b = BufMutB { data: vec![0; 15], position: 0 };

    let mut chain = Chain { a, b };

    let chunk = chain.chunk_mut();
    assert!(chunk.0.len() > 0);
}

