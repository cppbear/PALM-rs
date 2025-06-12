fn copy_to_bytes(&mut self, len: usize) -> crate::Bytes {
        let a_rem = self.a.remaining();
        if a_rem >= len {
            self.a.copy_to_bytes(len)
        } else if a_rem == 0 {
            self.b.copy_to_bytes(len)
        } else {
            assert!(
                len - a_rem <= self.b.remaining(),
                "`len` greater than remaining"
            );
            let mut ret = crate::BytesMut::with_capacity(len);
            ret.put(&mut self.a);
            ret.put((&mut self.b).take(len - a_rem));
            ret.freeze()
        }
    }