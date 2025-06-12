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