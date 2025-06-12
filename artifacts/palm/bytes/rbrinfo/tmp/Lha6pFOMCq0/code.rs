fn try_unsplit(&mut self, other: BytesMut) -> Result<(), BytesMut> {
        if other.capacity() == 0 {
            return Ok(());
        }

        let ptr = unsafe { self.ptr.as_ptr().add(self.len) };
        if ptr == other.ptr.as_ptr()
            && self.kind() == KIND_ARC
            && other.kind() == KIND_ARC
            && self.data == other.data
        {
            // Contiguous blocks, just combine directly
            self.len += other.len;
            self.cap += other.cap;
            Ok(())
        } else {
            Err(other)
        }
    }