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
impl DecodeEstimate for GeneralPurposeEstimate {
    fn decoded_len_estimate(&self) -> usize {
        self.conservative_decoded_len
    }
}
