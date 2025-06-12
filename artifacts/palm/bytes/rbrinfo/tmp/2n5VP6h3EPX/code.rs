pub fn split(&mut self) -> BytesMut {
        let len = self.len();
        self.split_to(len)
    }