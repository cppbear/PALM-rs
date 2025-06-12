fn reserve_inner(&mut self, additional: usize, allocate: bool) -> bool {
        let len = self.len();
        let kind = self.kind();

        if kind == KIND_VEC {
            // If there's enough free space before the start of the buffer, then
            // just copy the data backwards and reuse the already-allocated
            // space.
            //
            // Otherwise, since backed by a vector, use `Vec::reserve`
            //
            // We need to make sure that this optimization does not kill the
            // amortized runtimes of BytesMut's operations.
            unsafe {
                let off = self.get_vec_pos();

                // Only reuse space if we can satisfy the requested additional space.
                //
                // Also check if the value of `off` suggests that enough bytes
                // have been read to account for the overhead of shifting all
                // the data (in an amortized analysis).
                // Hence the condition `off >= self.len()`.
                //
                // This condition also already implies that the buffer is going
                // to be (at least) half-empty in the end; so we do not break
                // the (amortized) runtime with future resizes of the underlying
                // `Vec`.
                //
                // [For more details check issue #524, and PR #525.]
                if self.capacity() - self.len() + off >= additional && off >= self.len() {
                    // There's enough space, and it's not too much overhead:
                    // reuse the space!
                    //
                    // Just move the pointer back to the start after copying
                    // data back.
                    let base_ptr = self.ptr.as_ptr().sub(off);
                    // Since `off >= self.len()`, the two regions don't overlap.
                    ptr::copy_nonoverlapping(self.ptr.as_ptr(), base_ptr, self.len);
                    self.ptr = vptr(base_ptr);
                    self.set_vec_pos(0);

                    // Length stays constant, but since we moved backwards we
                    // can gain capacity back.
                    self.cap += off;
                } else {
                    if !allocate {
                        return false;
                    }
                    // Not enough space, or reusing might be too much overhead:
                    // allocate more space!
                    let mut v =
                        ManuallyDrop::new(rebuild_vec(self.ptr.as_ptr(), self.len, self.cap, off));
                    v.reserve(additional);

                    // Update the info
                    self.ptr = vptr(v.as_mut_ptr().add(off));
                    self.cap = v.capacity() - off;
                    debug_assert_eq!(self.len, v.len() - off);
                }

                return true;
            }
        }

        debug_assert_eq!(kind, KIND_ARC);
        let shared: *mut Shared = self.data;

        // Reserving involves abandoning the currently shared buffer and
        // allocating a new vector with the requested capacity.
        //
        // Compute the new capacity
        let mut new_cap = match len.checked_add(additional) {
            Some(new_cap) => new_cap,
            None if !allocate => return false,
            None => panic!("overflow"),
        };

        unsafe {
            // First, try to reclaim the buffer. This is possible if the current
            // handle is the only outstanding handle pointing to the buffer.
            if (*shared).is_unique() {
                // This is the only handle to the buffer. It can be reclaimed.
                // However, before doing the work of copying data, check to make
                // sure that the vector has enough capacity.
                let v = &mut (*shared).vec;

                let v_capacity = v.capacity();
                let ptr = v.as_mut_ptr();

                let offset = offset_from(self.ptr.as_ptr(), ptr);

                // Compare the condition in the `kind == KIND_VEC` case above
                // for more details.
                if v_capacity >= new_cap + offset {
                    self.cap = new_cap;
                    // no copy is necessary
                } else if v_capacity >= new_cap && offset >= len {
                    // The capacity is sufficient, and copying is not too much
                    // overhead: reclaim the buffer!

                    // `offset >= len` means: no overlap
                    ptr::copy_nonoverlapping(self.ptr.as_ptr(), ptr, len);

                    self.ptr = vptr(ptr);
                    self.cap = v.capacity();
                } else {
                    if !allocate {
                        return false;
                    }
                    // calculate offset
                    let off = (self.ptr.as_ptr() as usize) - (v.as_ptr() as usize);

                    // new_cap is calculated in terms of `BytesMut`, not the underlying
                    // `Vec`, so it does not take the offset into account.
                    //
                    // Thus we have to manually add it here.
                    new_cap = new_cap.checked_add(off).expect("overflow");

                    // The vector capacity is not sufficient. The reserve request is
                    // asking for more than the initial buffer capacity. Allocate more
                    // than requested if `new_cap` is not much bigger than the current
                    // capacity.
                    //
                    // There are some situations, using `reserve_exact` that the
                    // buffer capacity could be below `original_capacity`, so do a
                    // check.
                    let double = v.capacity().checked_shl(1).unwrap_or(new_cap);

                    new_cap = cmp::max(double, new_cap);

                    // No space - allocate more
                    //
                    // The length field of `Shared::vec` is not used by the `BytesMut`;
                    // instead we use the `len` field in the `BytesMut` itself. However,
                    // when calling `reserve`, it doesn't guarantee that data stored in
                    // the unused capacity of the vector is copied over to the new
                    // allocation, so we need to ensure that we don't have any data we
                    // care about in the unused capacity before calling `reserve`.
                    debug_assert!(off + len <= v.capacity());
                    v.set_len(off + len);
                    v.reserve(new_cap - v.len());

                    // Update the info
                    self.ptr = vptr(v.as_mut_ptr().add(off));
                    self.cap = v.capacity() - off;
                }

                return true;
            }
        }
        if !allocate {
            return false;
        }

        let original_capacity_repr = unsafe { (*shared).original_capacity_repr };
        let original_capacity = original_capacity_from_repr(original_capacity_repr);

        new_cap = cmp::max(new_cap, original_capacity);

        // Create a new vector to store the data
        let mut v = ManuallyDrop::new(Vec::with_capacity(new_cap));

        // Copy the bytes
        v.extend_from_slice(self.as_ref());

        // Release the shared handle. This must be done *after* the bytes are
        // copied.
        unsafe { release_shared(shared) };

        // Update self
        let data = (original_capacity_repr << ORIGINAL_CAPACITY_OFFSET) | KIND_VEC;
        self.data = invalid_ptr(data);
        self.ptr = vptr(v.as_mut_ptr());
        self.cap = v.capacity();
        debug_assert_eq!(self.len, v.len());
        return true;
    }