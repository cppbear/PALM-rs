unsafe fn rehash_in_place(
        &mut self,
        hasher: &dyn Fn(&mut Self, usize) -> u64,
        size_of: usize,
        drop: Option<unsafe fn(*mut u8)>,
    ) {
        // If the hash function panics then properly clean up any elements
        // that we haven't rehashed yet. We unfortunately can't preserve the
        // element since we lost their hash and have no way of recovering it
        // without risking another panic.
        self.prepare_rehash_in_place();

        let mut guard = guard(self, move |self_| {
            if let Some(drop) = drop {
                for i in 0..self_.buckets() {
                    if *self_.ctrl(i) == Tag::DELETED {
                        self_.set_ctrl(i, Tag::EMPTY);
                        drop(self_.bucket_ptr(i, size_of));
                        self_.items -= 1;
                    }
                }
            }
            self_.growth_left = bucket_mask_to_capacity(self_.bucket_mask) - self_.items;
        });

        // At this point, DELETED elements are elements that we haven't
        // rehashed yet. Find them and re-insert them at their ideal
        // position.
        'outer: for i in 0..guard.buckets() {
            if *guard.ctrl(i) != Tag::DELETED {
                continue;
            }

            let i_p = guard.bucket_ptr(i, size_of);

            'inner: loop {
                // Hash the current item
                let hash = hasher(*guard, i);

                // Search for a suitable place to put it
                //
                // SAFETY: Caller of this function ensures that the control bytes
                // are properly initialized.
                let new_i = guard.find_insert_slot(hash).index;

                // Probing works by scanning through all of the control
                // bytes in groups, which may not be aligned to the group
                // size. If both the new and old position fall within the
                // same unaligned group, then there is no benefit in moving
                // it and we can just continue to the next item.
                if likely(guard.is_in_same_group(i, new_i, hash)) {
                    guard.set_ctrl_hash(i, hash);
                    continue 'outer;
                }

                let new_i_p = guard.bucket_ptr(new_i, size_of);

                // We are moving the current item to a new position. Write
                // our H2 to the control byte of the new position.
                let prev_ctrl = guard.replace_ctrl_hash(new_i, hash);
                if prev_ctrl == Tag::EMPTY {
                    guard.set_ctrl(i, Tag::EMPTY);
                    // If the target slot is empty, simply move the current
                    // element into the new slot and clear the old control
                    // byte.
                    ptr::copy_nonoverlapping(i_p, new_i_p, size_of);
                    continue 'outer;
                } else {
                    // If the target slot is occupied, swap the two elements
                    // and then continue processing the element that we just
                    // swapped into the old slot.
                    debug_assert_eq!(prev_ctrl, Tag::DELETED);
                    ptr::swap_nonoverlapping(i_p, new_i_p, size_of);
                    continue 'inner;
                }
            }
        }

        guard.growth_left = bucket_mask_to_capacity(guard.bucket_mask) - guard.items;

        mem::forget(guard);
    }