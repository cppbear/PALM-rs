unsafe fn shallow_clone(&mut self) -> BytesMut {
        if self.kind() == KIND_ARC {
            increment_shared(self.data);
            ptr::read(self)
        } else {
            self.promote_to_shared(/*ref_count = */ 2);
            ptr::read(self)
        }
    }