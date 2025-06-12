fn drop(&mut self) {
        let kind = self.kind();

        if kind == KIND_VEC {
            unsafe {
                let off = self.get_vec_pos();

                // Vector storage, free the vector
                let _ = rebuild_vec(self.ptr.as_ptr(), self.len, self.cap, off);
            }
        } else if kind == KIND_ARC {
            unsafe { release_shared(self.data) };
        }
    }