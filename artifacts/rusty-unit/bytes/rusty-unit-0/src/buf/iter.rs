use crate::Buf;

/// Iterator over the bytes contained by the buffer.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// use bytes::Bytes;
///
/// let buf = Bytes::from(&b"abc"[..]);
/// let mut iter = buf.into_iter();
///
/// assert_eq!(iter.next(), Some(b'a'));
/// assert_eq!(iter.next(), Some(b'b'));
/// assert_eq!(iter.next(), Some(b'c'));
/// assert_eq!(iter.next(), None);
/// ```
#[derive(Debug)]
pub struct IntoIter<T> {
    inner: T,
}

impl<T> IntoIter<T> {
    /// Creates an iterator over the bytes contained by the buffer.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytes::Bytes;
    ///
    /// let buf = Bytes::from_static(b"abc");
    /// let mut iter = buf.into_iter();
    ///
    /// assert_eq!(iter.next(), Some(b'a'));
    /// assert_eq!(iter.next(), Some(b'b'));
    /// assert_eq!(iter.next(), Some(b'c'));
    /// assert_eq!(iter.next(), None);
    /// ```
    pub fn new(inner: T) -> IntoIter<T> {
        IntoIter { inner }
    }

    /// Consumes this `IntoIter`, returning the underlying value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use bytes::{Buf, Bytes};
    ///
    /// let buf = Bytes::from(&b"abc"[..]);
    /// let mut iter = buf.into_iter();
    ///
    /// assert_eq!(iter.next(), Some(b'a'));
    ///
    /// let buf = iter.into_inner();
    /// assert_eq!(2, buf.remaining());
    /// ```
    pub fn into_inner(self) -> T {
        self.inner
    }

    /// Gets a reference to the underlying `Buf`.
    ///
    /// It is inadvisable to directly read from the underlying `Buf`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use bytes::{Buf, Bytes};
    ///
    /// let buf = Bytes::from(&b"abc"[..]);
    /// let mut iter = buf.into_iter();
    ///
    /// assert_eq!(iter.next(), Some(b'a'));
    ///
    /// assert_eq!(2, iter.get_ref().remaining());
    /// ```
    pub fn get_ref(&self) -> &T {
        &self.inner
    }

    /// Gets a mutable reference to the underlying `Buf`.
    ///
    /// It is inadvisable to directly read from the underlying `Buf`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use bytes::{Buf, BytesMut};
    ///
    /// let buf = BytesMut::from(&b"abc"[..]);
    /// let mut iter = buf.into_iter();
    ///
    /// assert_eq!(iter.next(), Some(b'a'));
    ///
    /// iter.get_mut().advance(1);
    ///
    /// assert_eq!(iter.next(), Some(b'c'));
    /// ```
    pub fn get_mut(&mut self) -> &mut T {
        &mut self.inner
    }
}

impl<T: Buf> Iterator for IntoIter<T> {
    type Item = u8;

    fn next(&mut self) -> Option<u8> {
        if !self.inner.has_remaining() {
            return None;
        }

        let b = self.inner.chunk()[0];
        self.inner.advance(1);

        Some(b)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let rem = self.inner.remaining();
        (rem, Some(rem))
    }
}

impl<T: Buf> ExactSizeIterator for IntoIter<T> {}

#[cfg(test)]
mod rusty_tests {
    use crate::*;
    use buf::buf_impl::Buf;
    use std::cmp::PartialEq;
    use std::cmp::PartialOrd;
    use std::convert::From;
    use std::default::Default;
    use std::fmt::Write;
    use std::iter::IntoIterator;
    use std::ops::DerefMut;
    use std::ops::Drop;
    // #[test]
    // #[should_panic]
    // #[timeout(3000)]
    // fn rusty_test_1() {
    //     rusty_monitor::set_test_id(1);
    //     let mut usize_0: usize = 632usize;
    //     let mut bytes_0: crate::bytes::Bytes = crate::bytes::Bytes::new();
    //     let mut bytesmut_0: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::from(bytes_0);
    //     let mut bytesmut_0_ref_0: &mut crate::bytes_mut::BytesMut = &mut bytesmut_0;
    //     let mut str_0: &str = "VX46f0vXnpnSgvaOGg";
    //     let mut str_0_ref_0: &str = &mut str_0;
    //     let mut bytes_1: crate::bytes::Bytes = crate::bytes::Bytes::from(str_0_ref_0);
    //     let mut bytesmut_1: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::from(bytes_1);
    //     let mut bytesmut_1_ref_0: &crate::bytes_mut::BytesMut = &mut bytesmut_1;
    //     let mut str_1: &str = "Hna";
    //     let mut str_1_ref_0: &str = &mut str_1;
    //     let mut bytesmut_2: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::new();
    //     let mut bytesmut_2_ref_0: &crate::bytes_mut::BytesMut = &mut bytesmut_2;
    //     let mut usize_1: usize = 4614usize;
    //     let mut bytesmut_3: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::with_capacity(usize_1);
    //     let mut bytesmut_3_ref_0: &crate::bytes_mut::BytesMut = &mut bytesmut_3;
    //     let mut usize_2: usize = 9267usize;
    //     let mut usize_3: usize = 2009usize;
    //     let mut bytesmut_4: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::new();
    //     let mut bytesmut_4_ref_0: &mut crate::bytes_mut::BytesMut = &mut bytesmut_4;
    //     let mut bytes_2: crate::bytes::Bytes = crate::bytes_mut::BytesMut::copy_to_bytes(bytesmut_4_ref_0, usize_3);
    //     let mut bytes_2_ref_0: &crate::bytes::Bytes = &mut bytes_2;
    //     let mut isize_0: isize = -6937isize;
    //     let mut intoiter_0: crate::buf::iter::IntoIter<isize> = crate::buf::iter::IntoIter {inner: isize_0};
    //     let mut bytesmut_5: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::default();
    //     let mut isize_1: isize = crate::buf::iter::IntoIter::into_inner(intoiter_0);
    //     let mut u8_slice_0: &[u8] = crate::bytes::Bytes::chunk(bytes_2_ref_0);
    //     let mut iter_0: std::slice::Iter<u8> = crate::bytes_mut::BytesMut::into_iter(bytesmut_3_ref_0);
    //     let mut bool_0: bool = crate::bytes_mut::BytesMut::eq(bytesmut_2_ref_0, str_1_ref_0);
    //     crate::bytes_mut::BytesMut::truncate(bytesmut_0_ref_0, usize_0);
    //     panic!("From RustyUnit with love");
    // }

    #[test]
    #[should_panic]
    #[timeout(3000)]
    fn rusty_test_10() {
        rusty_monitor::set_test_id(10);
        let mut usize_0: usize = 2910usize;
        let mut isize_0: isize = 9499isize;
        let mut intoiter_0: crate::buf::iter::IntoIter<isize> =
            crate::buf::iter::IntoIter { inner: isize_0 };
        let mut str_0: &str = "";
        let mut str_0_ref_0: &str = &mut str_0;
        let mut bytesmut_0: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::new();
        let mut bytesmut_0_ref_0: &crate::bytes_mut::BytesMut = &mut bytesmut_0;
        let mut usize_1: usize = 6054usize;
        let mut usize_2: usize = 2562usize;
        let mut trygeterror_0: crate::TryGetError = crate::TryGetError {
            requested: usize_2,
            available: usize_1,
        };
        let mut result_0: std::result::Result<f64, crate::TryGetError> =
            std::result::Result::Err(trygeterror_0);
        let mut writer_0: crate::buf::writer::Writer<std::result::Result<f64, crate::TryGetError>> =
            crate::buf::writer::new(result_0);
        let mut writer_0_ref_0: &crate::buf::writer::Writer<
            std::result::Result<f64, crate::TryGetError>,
        > = &mut writer_0;
        let mut isize_1: isize = 7525isize;
        let mut usize_3: usize = 7661usize;
        let mut bytes_0: crate::bytes::Bytes = crate::bytes::Bytes::default();
        let mut bytes_0_ref_0: &mut crate::bytes::Bytes = &mut bytes_0;
        let mut str_1: &str = "2F";
        let mut str_1_ref_0: &str = &mut str_1;
        let mut str_2: &str = "Ro7vRTiR";
        let mut str_2_ref_0: &str = &mut str_2;
        let mut bytesmut_1: crate::bytes_mut::BytesMut =
            crate::bytes_mut::BytesMut::from(str_2_ref_0);
        let mut bytesmut_1_ref_0: &mut crate::bytes_mut::BytesMut = &mut bytesmut_1;
        let mut isize_2: isize = 14982isize;
        let mut reader_0: crate::buf::reader::Reader<isize> = crate::buf::reader::new(isize_2);
        crate::bytes_mut::BytesMut::write_str(bytesmut_1_ref_0, str_1_ref_0);
        crate::bytes::Bytes::advance(bytes_0_ref_0, usize_3);
        let mut intoiter_1: crate::buf::iter::IntoIter<isize> =
            crate::buf::iter::IntoIter { inner: isize_1 };
        let mut option_0: std::option::Option<std::cmp::Ordering> =
            crate::bytes_mut::BytesMut::partial_cmp(bytesmut_0_ref_0, str_0_ref_0);
        let mut isize_3: isize = crate::buf::iter::IntoIter::into_inner(intoiter_0);
        let mut bytesmut_2: crate::bytes_mut::BytesMut =
            crate::bytes_mut::BytesMut::with_capacity(usize_0);
        panic!("From RustyUnit with love");
    }

    // #[test]
    // #[should_panic]
    // #[timeout(3000)]
    // fn rusty_test_16() {
    //     rusty_monitor::set_test_id(16);
    //     let mut str_0: &str = "x1wyaO";
    //     let mut str_0_ref_0: &str = &mut str_0;
    //     let mut bytes_0: crate::bytes::Bytes = crate::bytes::Bytes::from(str_0_ref_0);
    //     let mut usize_0: usize = 243usize;
    //     let mut bytesmut_0: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::new();
    //     let mut bytesmut_0_ref_0: &crate::bytes_mut::BytesMut = &mut bytesmut_0;
    //     let mut bytesmut_1: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::default();
    //     let mut bytesmut_1_ref_0: &mut crate::bytes_mut::BytesMut = &mut bytesmut_1;
    //     let mut bytesmut_2: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::new();
    //     let mut bytesmut_2_ref_0: &mut crate::bytes_mut::BytesMut = &mut bytesmut_2;
    //     let mut bytes_1: crate::bytes::Bytes = crate::bytes::Bytes::new();
    //     let mut bytesmut_3: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::from(bytes_1);
    //     let mut bytesmut_3_ref_0: &crate::bytes_mut::BytesMut = &mut bytesmut_3;
    //     let mut usize_1: usize = 66usize;
    //     let mut usize_2: usize = 6996usize;
    //     let mut usize_3: usize = 6053usize;
    //     let mut usize_4: usize = 7853usize;
    //     let mut trygeterror_0: crate::TryGetError = crate::TryGetError {requested: usize_4, available: usize_3};
    //     let mut result_0: std::result::Result<u16, crate::TryGetError> = std::result::Result::Err(trygeterror_0);
    //     let mut intoiter_0: crate::buf::iter::IntoIter<std::result::Result<u16, crate::TryGetError>> = crate::buf::iter::IntoIter {inner: result_0};
    //     let mut intoiter_0_ref_0: &crate::buf::iter::IntoIter<std::result::Result<u16, crate::TryGetError>> = &mut intoiter_0;
    //     let mut isize_0: isize = 18723isize;
    //     let mut intoiter_1: crate::buf::iter::IntoIter<isize> = crate::buf::iter::IntoIter::new(isize_0);
    //     let mut isize_1: isize = crate::buf::iter::IntoIter::into_inner(intoiter_1);
    //     let mut bool_0: bool = crate::bytes_mut::BytesMut::is_empty(bytesmut_3_ref_0);
    //     let mut bytesmut_4: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::new();
    //     let mut bytesmut_4_ref_0: &mut crate::bytes_mut::BytesMut = &mut bytesmut_4;
    //     let mut u8_slice_0: &mut [u8] = crate::bytes_mut::BytesMut::deref_mut(bytesmut_4_ref_0);
    //     crate::bytes_mut::BytesMut::drop(bytesmut_1_ref_0);
    //     let mut iter_0: std::slice::Iter<u8> = crate::bytes_mut::BytesMut::into_iter(bytesmut_2_ref_0);
    //     let mut intoiter_2: crate::buf::iter::IntoIter<crate::bytes::Bytes> = crate::bytes::Bytes::into_iter(bytes_0);
    //     panic!("From RustyUnit with love");
    // }
}
