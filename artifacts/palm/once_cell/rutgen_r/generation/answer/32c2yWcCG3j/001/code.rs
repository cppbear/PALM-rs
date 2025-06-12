// Answer 0

#[test]
fn test_get_unchecked_with_initialized_value() {
    use std::num::NonZeroUsize;
    use std::sync::atomic::{AtomicUsize, Ordering};

    struct InitializedCell {
        inner: AtomicUsize,
    }

    impl InitializedCell {
        fn new(value: NonZeroUsize) -> Self {
            let cell = InitializedCell {
                inner: AtomicUsize::new(value.get()),
            };
            cell
        }

        unsafe fn get_unchecked(&self) -> NonZeroUsize {
            #[inline(always)]
            fn as_const_ptr(r: &AtomicUsize) -> *const usize {
                use core::mem::align_of;

                let p: *const AtomicUsize = r;
                const _ALIGNMENT_COMPATIBLE: () = assert!(align_of::<AtomicUsize>() % align_of::<usize>() == 0);
                p.cast::<usize>()
            }

            let p = as_const_ptr(&self.inner);

            let val = unsafe { p.read() };
            unsafe { NonZeroUsize::new_unchecked(val) }
        }
    }

    let non_zero_value = NonZeroUsize::new(42).unwrap();
    let cell = InitializedCell::new(non_zero_value);

    let result = unsafe { cell.get_unchecked() };
    assert_eq!(result.get(), 42);
}

#[test]
#[should_panic]
fn test_get_unchecked_with_zero_value() {
    use std::num::NonZeroUsize;
    use std::sync::atomic::{AtomicUsize, Ordering};

    struct UninitializedCell {
        inner: AtomicUsize,
    }

    impl UninitializedCell {
        fn new() -> Self {
            UninitializedCell {
                inner: AtomicUsize::new(0),
            }
        }

        unsafe fn get_unchecked(&self) -> NonZeroUsize {
            #[inline(always)]
            fn as_const_ptr(r: &AtomicUsize) -> *const usize {
                use core::mem::align_of;

                let p: *const AtomicUsize = r;
                const _ALIGNMENT_COMPATIBLE: () = assert!(align_of::<AtomicUsize>() % align_of::<usize>() == 0);
                p.cast::<usize>()
            }

            let p = as_const_ptr(&self.inner);
            let val = unsafe { p.read() };
            unsafe { NonZeroUsize::new_unchecked(val) } // This will panic if val is 0
        }
    }

    let cell = UninitializedCell::new();

    let _result = unsafe { cell.get_unchecked() }; // Should panic here
}

#[test]
fn test_get_unchecked_with_max_value() {
    use std::num::NonZeroUsize;
    use std::sync::atomic::{AtomicUsize, Ordering};

    struct MaxValueCell {
        inner: AtomicUsize,
    }

    impl MaxValueCell {
        fn new(value: NonZeroUsize) -> Self {
            MaxValueCell {
                inner: AtomicUsize::new(value.get()),
            }
        }

        unsafe fn get_unchecked(&self) -> NonZeroUsize {
            #[inline(always)]
            fn as_const_ptr(r: &AtomicUsize) -> *const usize {
                use core::mem::align_of;

                let p: *const AtomicUsize = r;
                const _ALIGNMENT_COMPATIBLE: () = assert!(align_of::<AtomicUsize>() % align_of::<usize>() == 0);
                p.cast::<usize>()
            }

            let p = as_const_ptr(&self.inner);
            let val = unsafe { p.read() };
            unsafe { NonZeroUsize::new_unchecked(val) }
        }
    }

    let max_value = NonZeroUsize::new(usize::MAX).unwrap();
    let cell = MaxValueCell::new(max_value);

    let result = unsafe { cell.get_unchecked() };
    assert_eq!(result.get(), usize::MAX);
}

