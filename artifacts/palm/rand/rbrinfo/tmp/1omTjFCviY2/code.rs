fn fill<R: Rng + ?Sized>(&mut self, rng: &mut R) {
                if self.len() > 0 {
                    let size = mem::size_of_val(self);
                    rng.fill_bytes(
                        // SAFETY: `self` non-null and valid for reads and writes within its `size`
                        // bytes. `self` meets the alignment requirements of `&mut [u8]`.
                        // The contents of `self` are initialized. Both `[u8]` and `[$t]` are valid
                        // for all bit-patterns of their contents (note that the SAFETY requirement
                        // on callers of this macro). `self` is not borrowed.
                        unsafe {
                            slice::from_raw_parts_mut(self.as_mut_ptr()
                                as *mut u8,
                                size
                            )
                        }
                    );
                    for x in self {
                        *x = x.to_le();
                    }
                }
            }