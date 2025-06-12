// Answer 0

#[test]
fn test_chain_remaining_mut_both_zero() {
    struct BufMutZero;

    unsafe impl BufMut for BufMutZero {
        fn remaining_mut(&self) -> usize {
            0
        }
        unsafe fn advance_mut(&mut self, _: usize) {}
        fn chunk_mut(&mut self) -> &mut UninitSlice {
            &mut UninitSlice::new_uninit(0)
        }
    }

    let buf_a = BufMutZero;
    let buf_b = BufMutZero;
    let chained = Chain { a: buf_a, b: buf_b };
    let _ = chained.remaining_mut();
}

#[test]
fn test_chain_remaining_mut_a_zero_b_non_zero() {
    struct BufMutNonZero;

    unsafe impl BufMut for BufMutNonZero {
        fn remaining_mut(&self) -> usize {
            5
        }
        unsafe fn advance_mut(&mut self, _: usize) {}
        fn chunk_mut(&mut self) -> &mut UninitSlice {
            &mut UninitSlice::new_uninit(0)
        }
    }

    let buf_a = BufMutZero;
    let buf_b = BufMutNonZero;
    let chained = Chain { a: buf_a, b: buf_b };
    let _ = chained.remaining_mut();
}

#[test]
fn test_chain_remaining_mut_a_non_zero_b_zero() {
    struct BufMutNonZero;

    unsafe impl BufMut for BufMutNonZero {
        fn remaining_mut(&self) -> usize {
            10
        }
        unsafe fn advance_mut(&mut self, _: usize) {}
        fn chunk_mut(&mut self) -> &mut UninitSlice {
            &mut UninitSlice::new_uninit(0)
        }
    }

    let buf_a = BufMutNonZero;
    let buf_b = BufMutZero;
    let chained = Chain { a: buf_a, b: buf_b };
    let _ = chained.remaining_mut();
}

#[test]
fn test_chain_remaining_mut_both_non_zero() {
    struct BufMutLarge;

    unsafe impl BufMut for BufMutLarge {
        fn remaining_mut(&self) -> usize {
            15
        }
        unsafe fn advance_mut(&mut self, _: usize) {}
        fn chunk_mut(&mut self) -> &mut UninitSlice {
            &mut UninitSlice::new_uninit(0)
        }
    }

    let buf_a = BufMutLarge;
    let buf_b = BufMutLarge;
    let chained = Chain { a: buf_a, b: buf_b };
    let _ = chained.remaining_mut();
}

#[test]
fn test_chain_remaining_mut_edge_case() {
    struct BufMutMax;

    unsafe impl BufMut for BufMutMax {
        fn remaining_mut(&self) -> usize {
            u32::MAX as usize
        }
        unsafe fn advance_mut(&mut self, _: usize) {}
        fn chunk_mut(&mut self) -> &mut UninitSlice {
            &mut UninitSlice::new_uninit(0)
        }
    }

    let buf_a = BufMutMax;
    let buf_b = BufMutMax;
    let chained = Chain { a: buf_a, b: buf_b };
    let _ = chained.remaining_mut();
}

