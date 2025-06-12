pub(crate) fn do_alloc<A: Allocator>(alloc: &A, layout: Layout) -> Result<NonNull<u8>, ()> {
        match alloc.allocate(layout) {
            Ok(ptr) => Ok(ptr.cast()),
            Err(_) => Err(()),
        }
    }