use crate::{
    engine::{
        general_purpose::INVALID_VALUE, DecodeEstimate, DecodeMetadata, DecodePaddingMode,
    },
    DecodeError, DecodeSliceError, PAD_BYTE,
};
pub trait DecodeEstimate {
    fn decoded_len_estimate(&self) -> usize;
}
pub struct GeneralPurposeEstimate {
    /// input len % 4
    rem: usize,
    conservative_decoded_len: usize,
}
impl GeneralPurposeEstimate {
    pub(crate) fn new(encoded_len: usize) -> Self {
        let rem = encoded_len % 4;
        Self {
            rem,
            conservative_decoded_len: (encoded_len / 4 + usize::from(rem > 0)) * 3,
        }
    }
}
