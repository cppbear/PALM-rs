fn copy_to_bytes(&mut self, len: usize) -> crate::Bytes {
        use super::BufMut;

        if self.remaining() < len {
            panic_advance(&TryGetError {
                requested: len,
                available: self.remaining(),
            });
        }

        let mut ret = crate::BytesMut::with_capacity(len);
        ret.put(self.take(len));
        ret.freeze()
    }