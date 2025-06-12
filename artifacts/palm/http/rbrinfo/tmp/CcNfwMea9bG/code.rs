pub fn to_str(&self) -> Result<&str, ToStrError> {
        let bytes = self.as_ref();

        for &b in bytes {
            if !is_visible_ascii(b) {
                return Err(ToStrError { _priv: () });
            }
        }

        unsafe { Ok(str::from_utf8_unchecked(bytes)) }
    }