// Answer 0

#[derive(Clone, Copy, Debug)]
struct NonZST;

impl NonZST {
    const IS_ZERO_SIZED: bool = false;
}

struct RawTable<T> {
    ptr: *mut T,
}

impl<T> RawTable<T> {
    fn new() -> Self {
        let boxed = Box::new(NonZST);
        Self {
            ptr: Box::into_raw(boxed),
        }
    }

    pub fn as_ptr(&self) -> *mut T {
        if T::IS_ZERO_SIZED {
            invalid_mut(std::mem::align_of::<T>())
        } else {
            unsafe { self.ptr.as_ptr().sub(1) }
        }
    }
}

#[test]
fn test_as_ptr_non_zero_sized() {
    let table: RawTable<NonZST> = RawTable::new();
    let ptr = table.as_ptr();
    assert!(!ptr.is_null());
}

#[test]
fn test_as_ptr_subtracts_one() {
    let table: RawTable<NonZST> = RawTable::new();
    let ptr = table.as_ptr();
    let expected_ptr = unsafe { table.ptr.as_ptr().sub(1) };
    assert_eq!(ptr, expected_ptr);
}

#[should_panic]
#[test]
fn test_as_ptr_panic_if_zst() {
    #[derive(Debug)]
    struct ZST;

    impl ZST {
        const IS_ZERO_SIZED: bool = true;
    }

    let table: RawTable<ZST> = RawTable::new();
    let _ = table.as_ptr(); // Should panic, as ZST is not allowed
}

