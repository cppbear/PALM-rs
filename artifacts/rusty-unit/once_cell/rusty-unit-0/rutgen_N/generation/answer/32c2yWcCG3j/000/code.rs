// Answer 0

#[cfg(test)]
mod tests {
    use super::*;
    use std::num::NonZeroUsize;
    use std::sync::atomic::{AtomicUsize, Ordering};

    struct RaceCell {
        inner: AtomicUsize,
    }

    impl RaceCell {
        fn new(value: NonZeroUsize) -> Self {
            RaceCell {
                inner: AtomicUsize::new(value.get()),
            }
        }

        unsafe fn get_unchecked(&self) -> NonZeroUsize {
            #[inline(always)]
            fn as_const_ptr(r: &AtomicUsize) -> *const usize {
                use core::mem::align_of;

                let p: *const AtomicUsize = r;
                const _ALIGNMENT_COMPATIBLE: () =
                    assert!(align_of::<AtomicUsize>() % align_of::<usize>() == 0);
                p.cast::<usize>()
            }

            let p = as_const_ptr(&self.inner);
            let val = unsafe { p.read() };
            unsafe { NonZeroUsize::new_unchecked(val) }
        }
    }

    #[test]
    fn test_get_unchecked() {
        let cell = RaceCell::new(NonZeroUsize::new(10).unwrap());
        unsafe {
            let value = cell.get_unchecked();
            assert_eq!(value.get(), 10);
        }
    }

    #[test]
    #[should_panic]
    fn test_get_unchecked_zero_value() {
        let cell = RaceCell { inner: AtomicUsize::new(0) };
        unsafe {
            cell.get_unchecked();
        }
    }
}

