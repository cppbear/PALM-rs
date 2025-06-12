// Extracted from the scopeguard crate
use core::{
    mem::ManuallyDrop,
    ops::{Deref, DerefMut},
    ptr,
};

pub struct ScopeGuard<T, F>
where
    F: FnMut(&mut T),
{
    dropfn: F,
    value: T,
}

#[inline]
pub fn guard<T, F>(value: T, dropfn: F) -> ScopeGuard<T, F>
where
    F: FnMut(&mut T),
{
    ScopeGuard { dropfn, value }
}

impl<T, F> ScopeGuard<T, F>
where
    F: FnMut(&mut T),
{
    #[inline]
    pub fn into_inner(guard: Self) -> T {
        // Cannot move out of Drop-implementing types, so
        // ptr::read the value out of a ManuallyDrop<Self>
        // Don't use mem::forget as that might invalidate value
        let guard = ManuallyDrop::new(guard);
        unsafe {
            let value = ptr::read(&guard.value);
            // read the closure so that it is dropped
            let _ = ptr::read(&guard.dropfn);
            value
        }
    }
}

impl<T, F> Deref for ScopeGuard<T, F>
where
    F: FnMut(&mut T),
{
    type Target = T;
    #[inline]
    fn deref(&self) -> &T {
        &self.value
    }
}

impl<T, F> DerefMut for ScopeGuard<T, F>
where
    F: FnMut(&mut T),
{
    #[inline]
    fn deref_mut(&mut self) -> &mut T {
        &mut self.value
    }
}

impl<T, F> Drop for ScopeGuard<T, F>
where
    F: FnMut(&mut T),
{
    #[inline]
    fn drop(&mut self) {
        (self.dropfn)(&mut self.value);
    }
}

#[cfg(test)]
mod tests_llm_16_124 {
    use super::*;

use crate::*;
    use scopeguard::ScopeGuard;

    #[test]
    fn test_deref() {
        let value = 42;
        let drop_fn = |_: &mut i32| {};
        let guard = ScopeGuard { dropfn: drop_fn, value };

        // Dereference the guard and check the value
        assert_eq!(*guard, 42);
    }
}

#[cfg(test)]
mod tests_llm_16_125 {
    use super::*;

use crate::*;
    use scopeguard::ScopeGuard;

    #[test]
    fn test_deref_mut() {
        let mut value = 10;
        let mut guard = ScopeGuard {
            dropfn: |v: &mut i32| *v = 0,
            value: value,
        };

        // Validate the deref_mut functionality
        let deref_mut_value = guard.deref_mut();
        *deref_mut_value += 5;

        // Ensure the original value reflects the modification
        assert_eq!(*deref_mut_value, 15);
        assert_eq!(guard.value, 15);
    }

    #[test]
    fn test_deref_mut_with_drop_fn() {
        let mut value = 10;
        {
            let mut guard = ScopeGuard {
                dropfn: |v: &mut i32| *v = 0,
                value: value,
            };

            // Modify the value using deref_mut
            *guard.deref_mut() += 5;
        }

        // Check that the drop function was called
        assert_eq!(value, 0);
    }
}

#[cfg(test)]
mod tests_llm_16_126 {
    use super::*;

use crate::*;
    use std::cell::RefCell;

    #[test]
    fn test_scope_guard_drop() {
        let value = RefCell::new(0);
        let mut dropped = false;

        {
            let _guard = ScopeGuard {
                dropfn: |v: &mut RefCell<i32>| {
                    *v.borrow_mut() += 1;
                    dropped = true;
                },
                value,
            };
        }

        assert!(dropped);
    }

    #[test]
    fn test_scope_guard_into_inner() {
        let value = RefCell::new(5);
        let mut dropped = false;

        let guard = ScopeGuard {
            dropfn: |v: &mut RefCell<i32>| {
                *v.borrow_mut() += 1;
                dropped = true;
            },
            value: RefCell::new(10),
        };

        let inner_value = ScopeGuard::into_inner(guard);
        assert_eq!(*inner_value.borrow(), 10);
        assert!(dropped);
    }
}
