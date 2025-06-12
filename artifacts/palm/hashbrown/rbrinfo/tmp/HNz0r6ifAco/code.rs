pub(crate) fn into_allocation(self) -> Option<(NonNull<u8>, Layout, A)> {
        let alloc = if self.table.is_empty_singleton() {
            None
        } else {
            // Avoid `Option::unwrap_or_else` because it bloats LLVM IR.
            let (layout, ctrl_offset) =
                match Self::TABLE_LAYOUT.calculate_layout_for(self.table.buckets()) {
                    Some(lco) => lco,
                    None => unsafe { hint::unreachable_unchecked() },
                };
            Some((
                unsafe { NonNull::new_unchecked(self.table.ctrl.as_ptr().sub(ctrl_offset).cast()) },
                layout,
                unsafe { ptr::read(&self.alloc) },
            ))
        };
        mem::forget(self);
        alloc
    }