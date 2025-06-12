fn with_capacity<A>(alloc: &A, table_layout: TableLayout, capacity: usize) -> Self
    where
        A: Allocator,
    {
        // Avoid `Result::unwrap_or_else` because it bloats LLVM IR.
        match Self::fallible_with_capacity(alloc, table_layout, capacity, Fallibility::Infallible) {
            Ok(table_inner) => table_inner,
            // SAFETY: All allocation errors will be caught inside `RawTableInner::new_uninitialized`.
            Err(_) => unsafe { hint::unreachable_unchecked() },
        }
    }