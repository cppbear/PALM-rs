// Answer 0

#[test]
fn test_remaining_mut_empty() {
    struct EmptyBuf;

    unsafe impl BufMut for EmptyBuf {
        fn remaining_mut(&self) -> usize {
            0
        }
        fn chunk_mut(&mut self) -> &mut UninitSlice {
            unimplemented!()
        }
        unsafe fn advance_mut(&mut self, _: usize) {}
    }

    let buf1 = EmptyBuf;
    let buf2 = EmptyBuf;
    let chain = Chain { a: buf1, b: buf2 };
    
    assert_eq!(chain.remaining_mut(), 0);
}

#[test]
fn test_remaining_mut_single_nonempty() {
    struct SimpleBuf(usize);

    unsafe impl BufMut for SimpleBuf {
        fn remaining_mut(&self) -> usize {
            self.0
        }
        fn chunk_mut(&mut self) -> &mut UninitSlice {
            unimplemented!()
        }
        unsafe fn advance_mut(&mut self, _: usize) {}
    }

    let buf1 = SimpleBuf(5);
    let buf2 = EmptyBuf;
    let chain = Chain { a: buf1, b: buf2 };
    
    assert_eq!(chain.remaining_mut(), 5);
}

#[test]
fn test_remaining_mut_both_nonempty() {
    struct SimpleBuf(usize);

    unsafe impl BufMut for SimpleBuf {
        fn remaining_mut(&self) -> usize {
            self.0
        }
        fn chunk_mut(&mut self) -> &mut UninitSlice {
            unimplemented!()
        }
        unsafe fn advance_mut(&mut self, _: usize) {}
    }

    let buf1 = SimpleBuf(5);
    let buf2 = SimpleBuf(10);
    let chain = Chain { a: buf1, b: buf2 };
    
    assert_eq!(chain.remaining_mut(), 15);
}

#[test]
fn test_remaining_mut_zero() {
    struct SimpleBuf(usize);

    unsafe impl BufMut for SimpleBuf {
        fn remaining_mut(&self) -> usize {
            self.0
        }
        fn chunk_mut(&mut self) -> &mut UninitSlice {
            unimplemented!()
        }
        unsafe fn advance_mut(&mut self, _: usize) {}
    }

    let buf1 = SimpleBuf(0);
    let buf2 = SimpleBuf(0);
    let chain = Chain { a: buf1, b: buf2 };
    
    assert_eq!(chain.remaining_mut(), 0);
}

#[test]
fn test_remaining_mut_with_saturation() {
    struct SimpleBuf(usize);

    unsafe impl BufMut for SimpleBuf {
        fn remaining_mut(&self) -> usize {
            self.0
        }
        fn chunk_mut(&mut self) -> &mut UninitSlice {
            unimplemented!()
        }
        unsafe fn advance_mut(&mut self, _: usize) {}
    }

    let buf1 = SimpleBuf(usize::MAX);
    let buf2 = SimpleBuf(1);
    let chain = Chain { a: buf1, b: buf2 };
    
    assert_eq!(chain.remaining_mut(), usize::MAX);
}

