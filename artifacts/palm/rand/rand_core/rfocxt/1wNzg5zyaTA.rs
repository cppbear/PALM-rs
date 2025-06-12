use crate::RngCore;
pub(crate) trait Observable: Copy {
    type Bytes: Sized + AsRef<[u8]>;
    fn to_le_bytes(self) -> Self::Bytes;
}
pub trait TryRngCore {
    type Error: fmt::Debug + fmt::Display;
    fn try_next_u32(&mut self) -> Result<u32, Self::Error>;
    fn try_next_u64(&mut self) -> Result<u64, Self::Error>;
    fn try_fill_bytes(&mut self, dst: &mut [u8]) -> Result<(), Self::Error>;
    fn unwrap_err(self) -> UnwrapErr<Self>
    where
        Self: Sized,
    {
        UnwrapErr(self)
    }
    fn unwrap_mut(&mut self) -> UnwrapMut<'_, Self> {
        UnwrapMut(self)
    }
    #[cfg(feature = "std")]
    fn read_adapter(&mut self) -> RngReadAdapter<'_, Self>
    where
        Self: Sized,
    {
        RngReadAdapter { inner: self }
    }
}
pub trait RngCore {
    fn next_u32(&mut self) -> u32;
    fn next_u64(&mut self) -> u64;
    fn fill_bytes(&mut self, dst: &mut [u8]);
}
impl Observable for u64 {
    type Bytes = [u8; 8];
    fn to_le_bytes(self) -> Self::Bytes {
        Self::to_le_bytes(self)
    }
}
