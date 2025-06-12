use core::{fmt, ops::DerefMut};
#[cfg(feature = "os_rng")]
pub use os::{OsError, OsRng};
pub trait RngCore {
    fn next_u32(&mut self) -> u32;
    fn next_u64(&mut self) -> u64;
    fn fill_bytes(&mut self, dst: &mut [u8]);
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
#[derive(Debug, Eq, PartialEq, Hash)]
pub struct UnwrapMut<'r, R: TryRngCore + ?Sized>(pub &'r mut R);
impl<'r, R: TryRngCore + ?Sized> UnwrapMut<'r, R> {
    #[inline(always)]
    pub fn re<'b>(&'b mut self) -> UnwrapMut<'b, R>
    where
        'r: 'b,
    {
        UnwrapMut(self.0)
    }
}
