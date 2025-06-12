unsafe fn clone_from_impl(&mut self, source: &Self) {
        // Copy the control bytes unchanged. We do this in a single pass
        source
            .table
            .ctrl(0)
            .copy_to_nonoverlapping(self.table.ctrl(0), self.table.num_ctrl_bytes());

        // The cloning of elements may panic, in which case we need
        // to make sure we drop only the elements that have been
        // cloned so far.
        let mut guard = guard((0, &mut *self), |(index, self_)| {
            if T::NEEDS_DROP {
                for i in 0..*index {
                    if self_.is_bucket_full(i) {
                        self_.bucket(i).drop();
                    }
                }
            }
        });

        for from in source.iter() {
            let index = source.bucket_index(&from);
            let to = guard.1.bucket(index);
            to.write(from.as_ref().clone());

            // Update the index in case we need to unwind.
            guard.0 = index + 1;
        }

        // Successfully cloned all items, no need to clean up.
        mem::forget(guard);

        self.table.items = source.table.items;
        self.table.growth_left = source.table.growth_left;
    }