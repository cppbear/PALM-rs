pub fn char(&mut self, range: impl RangeBounds<char>) -> char {
        let panic_empty_range = || {
            panic!(
                "empty range: {:?}..{:?}",
                range.start_bound(),
                range.end_bound()
            )
        };

        let surrogate_start = 0xd800u32;
        let surrogate_len = 0x800u32;

        let low = match range.start_bound() {
            Bound::Unbounded => 0u8 as char,
            Bound::Included(&x) => x,
            Bound::Excluded(&x) => {
                let scalar = if x as u32 == surrogate_start - 1 {
                    surrogate_start + surrogate_len
                } else {
                    x as u32 + 1
                };
                char::try_from(scalar).unwrap_or_else(|_| panic_empty_range())
            }
        };

        let high = match range.end_bound() {
            Bound::Unbounded => core::char::MAX,
            Bound::Included(&x) => x,
            Bound::Excluded(&x) => {
                let scalar = if x as u32 == surrogate_start + surrogate_len {
                    surrogate_start - 1
                } else {
                    (x as u32).wrapping_sub(1)
                };
                char::try_from(scalar).unwrap_or_else(|_| panic_empty_range())
            }
        };

        if low > high {
            panic_empty_range();
        }

        let gap = if (low as u32) < surrogate_start && (high as u32) >= surrogate_start {
            surrogate_len
        } else {
            0
        };
        let range = high as u32 - low as u32 - gap;
        let mut val = self.u32(0..=range) + low as u32;
        if val >= surrogate_start {
            val += gap;
        }
        val.try_into().unwrap()
    }