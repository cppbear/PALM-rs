fn hash<H: Hasher>(&self, hasher: &mut H) {
        if self.lower {
            hasher.write(self.buf);
        } else {
            for &b in self.buf {
                hasher.write(&[HEADER_CHARS[b as usize]]);
            }
        }
    }