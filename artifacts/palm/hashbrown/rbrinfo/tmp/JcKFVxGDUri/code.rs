unsafe fn allocation_info(&self, table_layout: TableLayout) -> (NonNull<u8>, Layout) {
        debug_assert!(
            !self.is_empty_singleton(),
            "this function can only be called on non-empty tables"
        );

        // Avoid `Option::unwrap_or_else` because it bloats LLVM IR.
        let (layout, ctrl_offset) = match table_layout.calculate_layout_for(self.buckets()) {
            Some(lco) => lco,
            None => unsafe { hint::unreachable_unchecked() },
        };
        (
            // SAFETY: The caller must uphold the safety contract for `allocation_info` method.
            unsafe { NonNull::new_unchecked(self.ctrl.as_ptr().sub(ctrl_offset)) },
            layout,
        )
    }