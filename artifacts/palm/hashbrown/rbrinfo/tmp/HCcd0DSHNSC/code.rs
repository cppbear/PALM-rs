fn clone_from(&mut self, source: &Self) {
        if source.table.is_empty_singleton() {
            let mut old_inner = mem::replace(&mut self.table, RawTableInner::NEW);
            unsafe {
                // SAFETY:
                // 1. We call the function only once;
                // 2. We know for sure that `alloc` and `table_layout` matches the [`Allocator`]
                //    and [`TableLayout`] that were used to allocate this table.
                // 3. If any elements' drop function panics, then there will only be a memory leak,
                //    because we have replaced the inner table with a new one.
                old_inner.drop_inner_table::<T, _>(&self.alloc, Self::TABLE_LAYOUT);
            }
        } else {
            unsafe {
                // Make sure that if any panics occurs, we clear the table and
                // leave it in an empty state.
                let mut self_ = guard(self, |self_| {
                    self_.clear_no_drop();
                });

                // First, drop all our elements without clearing the control
                // bytes. If this panics then the scope guard will clear the
                // table, leaking any elements that were not dropped yet.
                //
                // This leak is unavoidable: we can't try dropping more elements
                // since this could lead to another panic and abort the process.
                //
                // SAFETY: If something gets wrong we clear our table right after
                // dropping the elements, so there is no double drop, since `items`
                // will be equal to zero.
                self_.table.drop_elements::<T>();

                // If necessary, resize our table to match the source.
                if self_.buckets() != source.buckets() {
                    let new_inner = match RawTableInner::new_uninitialized(
                        &self_.alloc,
                        Self::TABLE_LAYOUT,
                        source.buckets(),
                        Fallibility::Infallible,
                    ) {
                        Ok(table) => table,
                        Err(_) => hint::unreachable_unchecked(),
                    };
                    // Replace the old inner with new uninitialized one. It's ok, since if something gets
                    // wrong `ScopeGuard` will initialize all control bytes and leave empty table.
                    let mut old_inner = mem::replace(&mut self_.table, new_inner);
                    if !old_inner.is_empty_singleton() {
                        // SAFETY:
                        // 1. We have checked that our table is allocated.
                        // 2. We know for sure that `alloc` and `table_layout` matches
                        // the [`Allocator`] and [`TableLayout`] that were used to allocate this table.
                        old_inner.free_buckets(&self_.alloc, Self::TABLE_LAYOUT);
                    }
                }

                // Cloning elements may fail (the clone function may panic), but the `ScopeGuard`
                // inside the `clone_from_impl` function will take care of that, dropping all
                // cloned elements if necessary. Our `ScopeGuard` will clear the table.
                self_.clone_from_spec(source);

                // Disarm the scope guard if cloning was successful.
                ScopeGuard::into_inner(self_);
            }
        }
    }