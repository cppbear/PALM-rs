pub(crate) fn match_tag(self, tag: Tag) -> BitMask {
        #[allow(
            clippy::cast_possible_wrap, // tag.0: Tag as i8
            // tag: i32 as u16
            //   note: _mm_movemask_epi8 returns a 16-bit mask in a i32, the
            //   upper 16-bits of the i32 are zeroed:
            clippy::cast_sign_loss,
            clippy::cast_possible_truncation
        )]
        unsafe {
            let cmp = x86::_mm_cmpeq_epi8(self.0, x86::_mm_set1_epi8(tag.0 as i8));
            BitMask(x86::_mm_movemask_epi8(cmp) as u16)
        }
    }