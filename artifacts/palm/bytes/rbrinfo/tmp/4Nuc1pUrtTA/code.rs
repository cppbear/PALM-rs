pub fn try_into_mut(self) -> Result<BytesMut, Bytes> {
        if self.is_unique() {
            Ok(self.into())
        } else {
            Err(self)
        }
    }