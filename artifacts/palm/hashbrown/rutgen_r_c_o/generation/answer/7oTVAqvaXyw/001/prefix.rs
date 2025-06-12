// Answer 0

use crate::raw::{Global, Allocator};
use std::fmt;

#[derive(Debug)]
struct TestAllocator;

unsafe impl Allocator for TestAllocator {
    fn allocate(&self, _: std::alloc::Layout) -> Result<std::ptr::NonNull<u8>, ()> {
        Ok(std::ptr::NonNull::new_unchecked(std::ptr::null_mut()))
    }
    unsafe fn deallocate(&self, _: std::ptr::NonNull<u8>, _: std::alloc::Layout) {}
}

struct CustomKey(u32);
struct CustomValue(u32);

#[test]
fn test_into_keys_empty() {
    let keys: Vec<CustomKey> = Vec::new();
    let values: Vec<CustomValue> = Vec::new();
    let allocator = TestAllocator;
    
    let into_keys = IntoKeys { inner: IntoIter { inner: RawIntoIter::new(keys.into_iter().zip(values.into_iter()), allocator) } };
    let _ = fmt::write(&mut fmt::Formatter::new(), format_args!("{:?}", into_keys));
}

#[test]
fn test_into_keys_single() {
    let keys = vec![CustomKey(1)];
    let values = vec![CustomValue(100)];
    let allocator = TestAllocator;
    
    let into_keys = IntoKeys { inner: IntoIter { inner: RawIntoIter::new(keys.into_iter().zip(values.into_iter()), allocator) } };
    let _ = fmt::write(&mut fmt::Formatter::new(), format_args!("{:?}", into_keys));
}

#[test]
fn test_into_keys_multiple() {
    let keys = (1..=10).map(CustomKey).collect::<Vec<_>>();
    let values = (1..=10).map(CustomValue).collect::<Vec<_>>();
    let allocator = TestAllocator;

    let into_keys = IntoKeys { inner: IntoIter { inner: RawIntoIter::new(keys.into_iter().zip(values.into_iter()), allocator) } };
    let _ = fmt::write(&mut fmt::Formatter::new(), format_args!("{:?}", into_keys));
}

#[test]
fn test_into_keys_max_items() {
    let keys: Vec<CustomKey> = (0..1000).map(CustomKey).collect();
    let values: Vec<CustomValue> = (0..1000).map(CustomValue).collect();
    let allocator = TestAllocator;

    let into_keys = IntoKeys { inner: IntoIter { inner: RawIntoIter::new(keys.into_iter().zip(values.into_iter()), allocator) } };
    let _ = fmt::write(&mut fmt::Formatter::new(), format_args!("{:?}", into_keys));
}

