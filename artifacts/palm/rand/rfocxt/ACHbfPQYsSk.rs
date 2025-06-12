use core::fmt;
pub use weighted_index::WeightedIndex;
macro_rules! impl_weight_int {
    ($t:ty) => {
        impl Weight for $t { const ZERO : Self = 0; fn checked_add_assign(& mut self, v :
        & Self) -> Result < (), () > { match self.checked_add(* v) { Some(sum) => { *
        self = sum; Ok(()) } None => Err(()), } } }
    };
    ($t:ty, $($tt:ty),*) => {
        impl_weight_int!($t); impl_weight_int!($($tt),*);
    };
}
impl_weight_int!(i8, i16, i32, i64, i128, isize);
impl_weight_int!(u8, u16, u32, u64, u128, usize);
macro_rules! impl_weight_float {
    ($t:ty) => {
        impl Weight for $t { const ZERO : Self = 0.0; fn checked_add_assign(& mut self, v
        : & Self) -> Result < (), () > { * self += * v; Ok(()) } }
    };
}
impl_weight_float!(f32);
impl_weight_float!(f64);
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum Error {
    /// The input weight sequence is empty, too long, or wrongly ordered
    InvalidInput,
    /// A weight is negative, too large for the distribution, or not a valid number
    InvalidWeight,
    /// Not enough non-zero weights are available to sample values
    ///
    /// When attempting to sample a single value this implies that all weights
    /// are zero. When attempting to sample `amount` values this implies that
    /// less than `amount` weights are greater than zero.
    InsufficientNonZero,
    /// Overflow when calculating the sum of weights
    Overflow,
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(
            match *self {
                Error::InvalidInput => "Weights sequence is empty/too long/unordered",
                Error::InvalidWeight => {
                    "A weight is negative, too large or not a valid number"
                }
                Error::InsufficientNonZero => "Not enough weights > zero",
                Error::Overflow => "Overflow when summing weights",
            },
        )
    }
}
