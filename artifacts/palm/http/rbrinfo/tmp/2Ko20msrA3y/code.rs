pub fn as_str(&self) -> &str {
        let offset = (self.0.get() - 100) as usize;
        let offset = offset * 3;

        // Invariant: self has checked range [100, 999] and CODE_DIGITS is
        // ASCII-only, of length 900 * 3 = 2700 bytes

        #[cfg(debug_assertions)]
        {
            &CODE_DIGITS[offset..offset + 3]
        }

        #[cfg(not(debug_assertions))]
        unsafe {
            CODE_DIGITS.get_unchecked(offset..offset + 3)
        }
    }