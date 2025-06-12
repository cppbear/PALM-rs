// FIXME: Branch prediction hint. This is currently only available on nightly
// but it consistently improves performance by 10-15%.
#[cfg(not(feature = "nightly"))]
pub(crate) use core::convert::{identity as likely, identity as unlikely};
#[cfg(feature = "nightly")]
pub(crate) use core::intrinsics::{likely, unlikely};

// FIXME: use strict provenance functions once they are stable.
// Implement it with a transmute for now.
#[inline(always)]
#[allow(clippy::useless_transmute)] // clippy is wrong, cast and transmute are different here
pub(crate) fn invalid_mut<T>(addr: usize) -> *mut T {
    unsafe { core::mem::transmute(addr) }
}

#[cfg(test)]
mod tests_llm_16_594 {
    use super::*;

use crate::*;
    use std::ptr;

    #[test]
    fn test_invalid_mut() {
        let addr: usize = 0x1 as usize; // Sample invalid address
        let ptr: *mut i32 = invalid_mut(addr);
        
        // Ensure that the pointer is the same as the address we passed
        assert_eq!(ptr as usize, addr);
        
        // Check that the pointer is non-null
        assert!(!ptr.is_null());
    }

    #[test]
    fn test_invalid_mut_different_types() {
        let addr: usize = 0x2 as usize; // Sample invalid address
        let ptr: *mut f64 = invalid_mut(addr);
        
        // Ensure that the pointer is the same as the address we passed
        assert_eq!(ptr as usize, addr);
        
        // Check that the pointer is non-null
        assert!(!ptr.is_null());
    }
}
