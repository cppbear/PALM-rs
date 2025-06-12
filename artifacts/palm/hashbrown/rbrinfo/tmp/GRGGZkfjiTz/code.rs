pub(crate) fn match_empty_or_deleted(self) -> BitMask {
        #[allow(
            // tag: i32 as u16
            //   note: _mm_movemask_epi8 returns a 16-bit mask in a i32, the
            //   upper 16-bits of the i32 are zeroed:
            clippy::cast_sign_loss,
            clippy::cast_possible_truncation
        )]
        unsafe {
            // A tag is EMPTY or DELETED iff the high bit is set
            BitMask(x86::_mm_movemask_epi8(self.0) as u16)
        }
    }