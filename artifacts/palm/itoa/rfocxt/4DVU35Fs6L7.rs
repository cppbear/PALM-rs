use core::hint;
use core::mem::MaybeUninit;
use core::{ptr, slice, str};
#[cfg(feature = "no-panic")]
use no_panic::no_panic;
const DEC_DIGITS_LUT: [u8; 200] = *b"\
      0001020304050607080910111213141516171819\
      2021222324252627282930313233343536373839\
      4041424344454647484950515253545556575859\
      6061626364656667686970717273747576777879\
      8081828384858687888990919293949596979899";
macro_rules! impl_Integer {
    ($t:ty [len = $max_len:expr] as $large_unsigned:ty) => {
        impl Integer for $t { const MAX_STR_LEN : usize = $max_len; } impl
        private::Sealed for $t { type Buffer = [MaybeUninit < u8 >; $max_len];
        #[allow(unused_comparisons)] #[inline] #[cfg_attr(feature = "no-panic",
        no_panic)] fn write(self, buf : & mut [MaybeUninit < u8 >; $max_len]) -> & str {
        let is_nonnegative = self >= 0; let mut n = if is_nonnegative { self as
        $large_unsigned } else { (! (self as $large_unsigned)).wrapping_add(1) }; let mut
        curr = buf.len(); let buf_ptr = buf.as_mut_ptr() as * mut u8; let lut_ptr =
        DEC_DIGITS_LUT.as_ptr(); while n >= 10000 { let rem = n % 10000; n /= 10000; let
        d1 = ((rem / 100) << 1) as usize; let d2 = ((rem % 100) << 1) as usize; curr -=
        4; unsafe { ptr::copy_nonoverlapping(lut_ptr.add(d1), buf_ptr.add(curr), 2);
        ptr::copy_nonoverlapping(lut_ptr.add(d2), buf_ptr.add(curr + 2), 2); } } if n >=
        100 { let d1 = ((n % 100) << 1) as usize; n /= 100; curr -= 2; unsafe {
        ptr::copy_nonoverlapping(lut_ptr.add(d1), buf_ptr.add(curr), 2); } } if n < 10 {
        curr -= 1; unsafe { * buf_ptr.add(curr) = (n as u8) + b'0'; } } else { let d1 =
        (n << 1) as usize; curr -= 2; unsafe { ptr::copy_nonoverlapping(lut_ptr.add(d1),
        buf_ptr.add(curr), 2); } } if ! is_nonnegative { curr -= 1; unsafe { * buf_ptr
        .add(curr) = b'-'; } } let len = buf.len() - curr; let bytes = unsafe {
        slice::from_raw_parts(buf_ptr.add(curr), len) }; unsafe {
        str::from_utf8_unchecked(bytes) } } }
    };
}
impl_Integer!(i8[len = 4] as u32);
impl_Integer!(u8[len = 3] as u32);
impl_Integer!(i16[len = 6] as u32);
impl_Integer!(u16[len = 5] as u32);
impl_Integer!(i32[len = 11] as u32);
impl_Integer!(u32[len = 10] as u32);
impl_Integer!(i64[len = 20] as u64);
impl_Integer!(u64[len = 20] as u64);
macro_rules! impl_Integer_size {
    ($t:ty as $primitive:ident #[cfg(target_pointer_width = $width:literal)]) => {
        #[cfg(target_pointer_width = $width)] impl Integer for $t { const MAX_STR_LEN :
        usize = <$primitive as Integer >::MAX_STR_LEN; } #[cfg(target_pointer_width =
        $width)] impl private::Sealed for $t { type Buffer = <$primitive as
        private::Sealed >::Buffer; #[inline] #[cfg_attr(feature = "no-panic", no_panic)]
        fn write(self, buf : & mut Self::Buffer) -> & str { (self as $primitive)
        .write(buf) } }
    };
}
impl_Integer_size!(isize as i16 #[cfg(target_pointer_width = "16")]);
impl_Integer_size!(usize as u16 #[cfg(target_pointer_width = "16")]);
impl_Integer_size!(isize as i32 #[cfg(target_pointer_width = "32")]);
impl_Integer_size!(usize as u32 #[cfg(target_pointer_width = "32")]);
impl_Integer_size!(isize as i64 #[cfg(target_pointer_width = "64")]);
impl_Integer_size!(usize as u64 #[cfg(target_pointer_width = "64")]);
macro_rules! impl_Integer128 {
    ($t:ty [len = $max_len:expr]) => {
        impl Integer for $t { const MAX_STR_LEN : usize = $max_len; } impl
        private::Sealed for $t { type Buffer = [MaybeUninit < u8 >; $max_len];
        #[allow(unused_comparisons)] #[inline] #[cfg_attr(feature = "no-panic",
        no_panic)] fn write(self, buf : & mut [MaybeUninit < u8 >; $max_len]) -> & str {
        let is_nonnegative = self >= 0; let n = if is_nonnegative { self as u128 } else {
        (! (self as u128)).wrapping_add(1) }; let mut curr = buf.len(); let buf_ptr = buf
        .as_mut_ptr() as * mut u8; let (n, rem) = udiv128::udivmod_1e19(n); let buf1 =
        unsafe { buf_ptr.add(curr - u64::MAX_STR_LEN) as * mut [MaybeUninit < u8 >;
        u64::MAX_STR_LEN] }; curr -= rem.write(unsafe { & mut * buf1 }).len(); if n != 0
        { let target = buf.len() - 19; unsafe { ptr::write_bytes(buf_ptr.add(target),
        b'0', curr - target); } curr = target; let (n, rem) = udiv128::udivmod_1e19(n);
        let buf2 = unsafe { buf_ptr.add(curr - u64::MAX_STR_LEN) as * mut [MaybeUninit <
        u8 >; u64::MAX_STR_LEN] }; curr -= rem.write(unsafe { & mut * buf2 }).len(); if n
        != 0 { let target = buf.len() - 38; unsafe { ptr::write_bytes(buf_ptr
        .add(target), b'0', curr - target); } curr = target; curr -= 1; unsafe { *
        buf_ptr.add(curr) = (n as u8) + b'0'; } } } if ! is_nonnegative { curr -= 1;
        unsafe { * buf_ptr.add(curr) = b'-'; } } let len = buf.len() - curr; let bytes =
        unsafe { slice::from_raw_parts(buf_ptr.add(curr), len) }; unsafe {
        str::from_utf8_unchecked(bytes) } } }
    };
}
impl_Integer128!(i128[len = 40]);
impl_Integer128!(u128[len = 39]);
pub struct Buffer {
    bytes: [MaybeUninit<u8>; i128::MAX_STR_LEN],
}
impl Buffer {
    #[inline]
    #[cfg_attr(feature = "no-panic", no_panic)]
    pub fn new() -> Buffer {
        let bytes = [MaybeUninit::<u8>::uninit(); i128::MAX_STR_LEN];
        Buffer { bytes }
    }
    #[cfg_attr(feature = "no-panic", no_panic)]
    pub fn format<I: Integer>(&mut self, i: I) -> &str {}
}
