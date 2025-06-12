pub fn as_str(&self) -> &str {
            let InlineExtension(ref data, len) = self;
            // Safety: the invariant of InlineExtension ensures that the first
            // len bytes of data contain valid UTF-8.
            unsafe { str::from_utf8_unchecked(&data[..*len as usize]) }
        }