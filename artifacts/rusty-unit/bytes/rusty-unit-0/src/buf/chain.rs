use crate::buf::{IntoIter, UninitSlice};
use crate::{Buf, BufMut};

#[cfg(feature = "std")]
use std::io::IoSlice;

/// A `Chain` sequences two buffers.
///
/// `Chain` is an adapter that links two underlying buffers and provides a
/// continuous view across both buffers. It is able to sequence either immutable
/// buffers ([`Buf`] values) or mutable buffers ([`BufMut`] values).
///
/// This struct is generally created by calling [`Buf::chain`]. Please see that
/// function's documentation for more detail.
///
/// # Examples
///
/// ```
/// use bytes::{Bytes, Buf};
///
/// let mut buf = (&b"hello "[..])
///     .chain(&b"world"[..]);
///
/// let full: Bytes = buf.copy_to_bytes(11);
/// assert_eq!(full[..], b"hello world"[..]);
/// ```
///
/// [`Buf::chain`]: Buf::chain
#[derive(Debug)]
pub struct Chain<T, U> {
    a: T,
    b: U,
}

impl<T, U> Chain<T, U> {
    /// Creates a new `Chain` sequencing the provided values.
    pub(crate) fn new(a: T, b: U) -> Chain<T, U> {
        Chain { a, b }
    }

    /// Gets a reference to the first underlying `Buf`.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytes::Buf;
    ///
    /// let buf = (&b"hello"[..])
    ///     .chain(&b"world"[..]);
    ///
    /// assert_eq!(buf.first_ref()[..], b"hello"[..]);
    /// ```
    pub fn first_ref(&self) -> &T {
        &self.a
    }

    /// Gets a mutable reference to the first underlying `Buf`.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytes::Buf;
    ///
    /// let mut buf = (&b"hello"[..])
    ///     .chain(&b"world"[..]);
    ///
    /// buf.first_mut().advance(1);
    ///
    /// let full = buf.copy_to_bytes(9);
    /// assert_eq!(full, b"elloworld"[..]);
    /// ```
    pub fn first_mut(&mut self) -> &mut T {
        &mut self.a
    }

    /// Gets a reference to the last underlying `Buf`.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytes::Buf;
    ///
    /// let buf = (&b"hello"[..])
    ///     .chain(&b"world"[..]);
    ///
    /// assert_eq!(buf.last_ref()[..], b"world"[..]);
    /// ```
    pub fn last_ref(&self) -> &U {
        &self.b
    }

    /// Gets a mutable reference to the last underlying `Buf`.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytes::Buf;
    ///
    /// let mut buf = (&b"hello "[..])
    ///     .chain(&b"world"[..]);
    ///
    /// buf.last_mut().advance(1);
    ///
    /// let full = buf.copy_to_bytes(10);
    /// assert_eq!(full, b"hello orld"[..]);
    /// ```
    pub fn last_mut(&mut self) -> &mut U {
        &mut self.b
    }

    /// Consumes this `Chain`, returning the underlying values.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytes::Buf;
    ///
    /// let chain = (&b"hello"[..])
    ///     .chain(&b"world"[..]);
    ///
    /// let (first, last) = chain.into_inner();
    /// assert_eq!(first[..], b"hello"[..]);
    /// assert_eq!(last[..], b"world"[..]);
    /// ```
    pub fn into_inner(self) -> (T, U) {
        (self.a, self.b)
    }
}

impl<T, U> Buf for Chain<T, U>
where
    T: Buf,
    U: Buf,
{
    fn remaining(&self) -> usize {
        self.a.remaining().saturating_add(self.b.remaining())
    }

    fn chunk(&self) -> &[u8] {
        if self.a.has_remaining() {
            self.a.chunk()
        } else {
            self.b.chunk()
        }
    }

    fn advance(&mut self, mut cnt: usize) {
        let a_rem = self.a.remaining();

        if a_rem != 0 {
            if a_rem >= cnt {
                self.a.advance(cnt);
                return;
            }

            // Consume what is left of a
            self.a.advance(a_rem);

            cnt -= a_rem;
        }

        self.b.advance(cnt);
    }

    #[cfg(feature = "std")]
    fn chunks_vectored<'a>(&'a self, dst: &mut [IoSlice<'a>]) -> usize {
        let mut n = self.a.chunks_vectored(dst);
        n += self.b.chunks_vectored(&mut dst[n..]);
        n
    }

    fn copy_to_bytes(&mut self, len: usize) -> crate::Bytes {
        let a_rem = self.a.remaining();
        if a_rem >= len {
            self.a.copy_to_bytes(len)
        } else if a_rem == 0 {
            self.b.copy_to_bytes(len)
        } else {
            assert!(
                len - a_rem <= self.b.remaining(),
                "`len` greater than remaining"
            );
            let mut ret = crate::BytesMut::with_capacity(len);
            ret.put(&mut self.a);
            ret.put((&mut self.b).take(len - a_rem));
            ret.freeze()
        }
    }
}

unsafe impl<T, U> BufMut for Chain<T, U>
where
    T: BufMut,
    U: BufMut,
{
    fn remaining_mut(&self) -> usize {
        self.a
            .remaining_mut()
            .saturating_add(self.b.remaining_mut())
    }

    fn chunk_mut(&mut self) -> &mut UninitSlice {
        if self.a.has_remaining_mut() {
            self.a.chunk_mut()
        } else {
            self.b.chunk_mut()
        }
    }

    unsafe fn advance_mut(&mut self, mut cnt: usize) {
        let a_rem = self.a.remaining_mut();

        if a_rem != 0 {
            if a_rem >= cnt {
                self.a.advance_mut(cnt);
                return;
            }

            // Consume what is left of a
            self.a.advance_mut(a_rem);

            cnt -= a_rem;
        }

        self.b.advance_mut(cnt);
    }
}

impl<T, U> IntoIterator for Chain<T, U>
where
    T: Buf,
    U: Buf,
{
    type Item = u8;
    type IntoIter = IntoIter<Chain<T, U>>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter::new(self)
    }
}

#[cfg(test)]
mod rusty_tests {
	use crate::*;
	use std::cmp::Ord;
	use std::cmp::PartialOrd;
	use std::ops::Deref;
	use std::ops::DerefMut;
	use std::iter::Extend;
	use std::iter::IntoIterator;
	use std::iter::FromIterator;
	use std::default::Default;
	use std::cmp::PartialEq;
	use std::clone::Clone;
	use buf::buf_impl::Buf;
	use std::convert::From;
	use std::cmp::Eq;
	use std::borrow::Borrow;
// #[test]
// #[should_panic]
// #[timeout(3000)]
// fn rusty_test_19() {
//     rusty_monitor::set_test_id(19);
//     let mut usize_0: usize = 9439usize;
//     let mut bytesmut_0: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::with_capacity(usize_0);
//     let mut bytesmut_0_ref_0: &crate::bytes_mut::BytesMut = &mut bytesmut_0;
//     let mut vec_0: std::vec::Vec<u8> = std::vec::Vec::new();
//     let mut bytes_0: crate::bytes::Bytes = crate::bytes::Bytes::from(vec_0);
//     let mut bytes_0_ref_0: &crate::bytes::Bytes = &mut bytes_0;
//     let mut usize_1: usize = 746usize;
//     let mut bytesmut_1: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::with_capacity(usize_1);
//     let mut bytesmut_1_ref_0: &crate::bytes_mut::BytesMut = &mut bytesmut_1;
//     let mut vec_1: std::vec::Vec<u8> = std::vec::Vec::new();
//     let mut bytesmut_2: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::from_iter::<std::vec::Vec<u8>>(vec_1);
//     let mut bytesmut_2_ref_0: &crate::bytes_mut::BytesMut = &mut bytesmut_2;
//     let mut vec_2: std::vec::Vec<u8> = std::vec::Vec::new();
//     let mut bytes_1: crate::bytes::Bytes = crate::bytes::Bytes::from(vec_2);
//     let mut bytes_1_ref_0: &crate::bytes::Bytes = &mut bytes_1;
//     let mut usize_2: usize = 9957usize;
//     let mut usize_3: usize = 517usize;
//     let mut trygeterror_0: crate::TryGetError = crate::TryGetError {requested: usize_3, available: usize_2};
//     let mut trygeterror_0_ref_0: &crate::TryGetError = &mut trygeterror_0;
//     let mut usize_4: usize = 6340usize;
//     let mut usize_5: usize = 2231usize;
//     let mut trygeterror_1: crate::TryGetError = crate::TryGetError {requested: usize_5, available: usize_4};
//     let mut trygeterror_1_ref_0: &crate::TryGetError = &mut trygeterror_1;
//     let mut bool_0: bool = crate::TryGetError::eq(trygeterror_1_ref_0, trygeterror_0_ref_0);
//     let mut bytes_2: crate::bytes::Bytes = crate::bytes::Bytes::clone(bytes_1_ref_0);
//     let mut ordering_0: std::cmp::Ordering = crate::bytes_mut::BytesMut::cmp(bytesmut_2_ref_0, bytesmut_1_ref_0);
//     let mut bytes_2_ref_0: &crate::bytes::Bytes = &mut bytes_2;
//     let mut iter_0: std::slice::Iter<u8> = crate::bytes::Bytes::into_iter(bytes_0_ref_0);
//     let mut iter_1: std::slice::Iter<u8> = crate::bytes::Bytes::into_iter(bytes_2_ref_0);
//     let mut u8_slice_0: &[u8] = crate::bytes_mut::BytesMut::deref(bytesmut_0_ref_0);
//     panic!("From RustyUnit with love");
// }

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_24() {
    rusty_monitor::set_test_id(24);
    let mut usize_0: usize = 3180usize;
    let mut bytesmut_0: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::with_capacity(usize_0);
    let mut isize_0: isize = 4865isize;
    let mut isize_1: isize = -3792isize;
    let mut chain_0: crate::buf::chain::Chain<isize, isize> = crate::buf::chain::Chain::new(isize_1, isize_0);
    let mut chain_0_ref_0: &crate::buf::chain::Chain<isize, isize> = &mut chain_0;
    let mut vec_0: std::vec::Vec<u8> = std::vec::Vec::new();
    let mut vec_1: std::vec::Vec<u8> = std::vec::Vec::new();
    let mut bytes_0: crate::bytes::Bytes = crate::bytes::Bytes::from(vec_1);
    let mut bytesmut_1: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::from(bytes_0);
    let mut bytesmut_1_ref_0: &mut crate::bytes_mut::BytesMut = &mut bytesmut_1;
    let mut isize_2: isize = 11359isize;
    let mut usize_1: usize = 6637usize;
    let mut usize_2: usize = 4484usize;
    let mut bytesmut_2: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::zeroed(usize_2);
    let mut bytesmut_2_ref_0: &mut crate::bytes_mut::BytesMut = &mut bytesmut_2;
    let mut vec_2: std::vec::Vec<u8> = std::vec::Vec::new();
    let mut bytes_1: crate::bytes::Bytes = crate::bytes::Bytes::from(vec_2);
    let mut bytes_1_ref_0: &crate::bytes::Bytes = &mut bytes_1;
    let mut vec_3: std::vec::Vec<u8> = std::vec::Vec::new();
    let mut str_0: &str = "YHytr9mO8SELPZtQV74";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut bytesmut_3: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::from(str_0_ref_0);
    let mut bytesmut_3_ref_0: &mut crate::bytes_mut::BytesMut = &mut bytesmut_3;
    crate::bytes_mut::BytesMut::extend(bytesmut_3_ref_0, vec_3);
    let mut u8_slice_0: &[u8] = crate::bytes::Bytes::borrow(bytes_1_ref_0);
    let mut u8_slice_1: &mut [u8] = crate::bytes_mut::BytesMut::deref_mut(bytesmut_2_ref_0);
    let mut writer_0: crate::buf::writer::Writer<isize> = crate::buf::writer::new(isize_2);
    crate::bytes_mut::BytesMut::extend(bytesmut_1_ref_0, vec_0);
    let mut isize_3: &isize = crate::buf::chain::Chain::last_ref(chain_0_ref_0);
    let mut bytes_2: crate::bytes::Bytes = crate::bytes_mut::BytesMut::freeze(bytesmut_0);
    panic!("From RustyUnit with love");
}

// #[test]
// #[should_panic]
// #[timeout(3000)]
// fn rusty_test_35() {
//     rusty_monitor::set_test_id(35);
//     let mut usize_0: usize = 2055usize;
//     let mut bytesmut_0: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::zeroed(usize_0);
//     let mut bytesmut_0_ref_0: &crate::bytes_mut::BytesMut = &mut bytesmut_0;
//     let mut usize_1: usize = 5509usize;
//     let mut usize_2: usize = 8269usize;
//     let mut isize_0: isize = 552isize;
//     let mut take_0: crate::buf::take::Take<isize> = crate::buf::take::new(isize_0, usize_2);
//     let mut take_0_ref_0: &crate::buf::take::Take<isize> = &mut take_0;
//     let mut usize_3: usize = 1691usize;
//     let mut bytesmut_1: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::with_capacity(usize_3);
//     let mut bytesmut_1_ref_0: &mut crate::bytes_mut::BytesMut = &mut bytesmut_1;
//     let mut isize_1: isize = 5713isize;
//     let mut isize_2: isize = 3811isize;
//     let mut chain_0: crate::buf::chain::Chain<isize, isize> = crate::buf::chain::Chain {a: isize_2, b: isize_1};
//     let mut chain_0_ref_0: &mut crate::buf::chain::Chain<isize, isize> = &mut chain_0;
//     let mut vec_0: std::vec::Vec<u8> = std::vec::Vec::new();
//     let mut bytesmut_2: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::from_iter::<std::vec::Vec<u8>>(vec_0);
//     let mut vec_1: std::vec::Vec<u8> = std::vec::Vec::new();
//     let mut bytes_0: crate::bytes::Bytes = crate::bytes::Bytes::from(vec_1);
//     let mut usize_4: usize = 94usize;
//     let mut vec_2: std::vec::Vec<u8> = std::vec::Vec::new();
//     let mut bytes_1: crate::bytes::Bytes = crate::bytes::Bytes::from(vec_2);
//     let mut bytes_1_ref_0: &crate::bytes::Bytes = &mut bytes_1;
//     let mut usize_5: usize = 1844usize;
//     let mut bytesmut_3: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::zeroed(usize_5);
//     let mut usize_6: usize = crate::bytes::Bytes::remaining(bytes_1_ref_0);
//     let mut intoiter_0: crate::buf::iter::IntoIter<crate::bytes::Bytes> = crate::bytes::Bytes::into_iter(bytes_0);
//     let mut bytesmut_3_ref_0: &crate::bytes_mut::BytesMut = &mut bytesmut_3;
//     let mut bytes_2: crate::bytes::Bytes = crate::bytes_mut::BytesMut::freeze(bytesmut_2);
//     let mut isize_3: &mut isize = crate::buf::chain::Chain::last_mut(chain_0_ref_0);
//     let mut intoiter_0_ref_0: &crate::buf::iter::IntoIter<isize> = &mut intoiter_0;
//     let mut isize_4: &isize = crate::buf::iter::IntoIter::get_ref(intoiter_0_ref_0);
//     let mut usize_7: usize = crate::buf::take::Take::limit(take_0_ref_0);
//     panic!("From RustyUnit with love");
// }

// #[test]
// #[should_panic]
// #[timeout(3000)]
// fn rusty_test_39() {
//     rusty_monitor::set_test_id(39);
//     let mut usize_0: usize = 1822usize;
//     let mut usize_1: usize = 8937usize;
//     let mut trygeterror_0: crate::TryGetError = crate::TryGetError {requested: usize_1, available: usize_0};
//     let mut trygeterror_0_ref_0: &crate::TryGetError = &mut trygeterror_0;
//     let mut usize_2: usize = 2504usize;
//     let mut bytesmut_0: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::zeroed(usize_2);
//     let mut usize_3: usize = 2038usize;
//     let mut usize_4: usize = 7647usize;
//     let mut bytesmut_1: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::zeroed(usize_4);
//     let mut bytesmut_1_ref_0: &mut crate::bytes_mut::BytesMut = &mut bytesmut_1;
//     let mut usize_5: usize = 6767usize;
//     let mut bytesmut_2: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::default();
//     let mut bytes_0: crate::bytes::Bytes = crate::bytes_mut::BytesMut::freeze(bytesmut_2);
//     let mut bytes_0_ref_0: &crate::bytes::Bytes = &mut bytes_0;
//     let mut bytesmut_3: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::default();
//     let mut bytes_1: crate::bytes::Bytes = crate::bytes_mut::BytesMut::freeze(bytesmut_3);
//     let mut bytes_1_ref_0: &crate::bytes::Bytes = &mut bytes_1;
//     let mut usize_6: usize = 8845usize;
//     let mut isize_0: isize = 755isize;
//     let mut isize_1: isize = -7732isize;
//     let mut chain_0: crate::buf::chain::Chain<isize, isize> = crate::buf::chain::Chain {a: isize_1, b: isize_0};
//     let mut chain_0_ref_0: &crate::buf::chain::Chain<isize, isize> = &mut chain_0;
//     let mut isize_2: &isize = crate::buf::chain::Chain::first_ref(chain_0_ref_0);
//     let mut bool_0: bool = crate::bytes::Bytes::eq(bytes_1_ref_0, bytes_0_ref_0);
//     let mut bytesmut_4: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::with_capacity(usize_5);
//     let mut bytesmut_4_ref_0: &crate::bytes_mut::BytesMut = &mut bytesmut_4;
//     let mut iter_0: std::slice::Iter<u8> = crate::bytes_mut::BytesMut::into_iter(bytesmut_4_ref_0);
//     let mut bytesmut_5: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::split_off(bytesmut_1_ref_0, usize_3);
//     let mut intoiter_0: crate::buf::iter::IntoIter<crate::bytes_mut::BytesMut> = crate::bytes_mut::BytesMut::into_iter(bytesmut_0);
//     let mut tuple_0: () = crate::TryGetError::assert_receiver_is_total_eq(trygeterror_0_ref_0);
//     panic!("From RustyUnit with love");
// }

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_40() {
    rusty_monitor::set_test_id(40);
    let mut bytes_0: crate::bytes::Bytes = crate::bytes::Bytes::new();
    let mut bytesmut_0: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::from(bytes_0);
    let mut bytesmut_0_ref_0: &crate::bytes_mut::BytesMut = &mut bytesmut_0;
    let mut usize_0: usize = 5382usize;
    let mut usize_1: usize = 4810usize;
    let mut usize_2: usize = 8591usize;
    let mut bytesmut_1: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::zeroed(usize_2);
    let mut bytesmut_1_ref_0: &mut crate::bytes_mut::BytesMut = &mut bytesmut_1;
    let mut isize_0: isize = 24882isize;
    let mut isize_1: isize = 10550isize;
    let mut chain_0: crate::buf::chain::Chain<isize, isize> = crate::buf::chain::Chain {a: isize_1, b: isize_0};
    let mut chain_0_ref_0: &mut crate::buf::chain::Chain<isize, isize> = &mut chain_0;
    let mut usize_3: usize = 3723usize;
    let mut usize_4: usize = 4571usize;
    let mut bytesmut_2: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::with_capacity(usize_4);
    let mut bytesmut_2_ref_0: &mut crate::bytes_mut::BytesMut = &mut bytesmut_2;
    let mut isize_2: isize = -11328isize;
    let mut isize_3: isize = -14403isize;
    let mut chain_1: crate::buf::chain::Chain<isize, isize> = crate::buf::chain::Chain {a: isize_3, b: isize_2};
    let mut chain_1_ref_0: &crate::buf::chain::Chain<isize, isize> = &mut chain_1;
    let mut vec_0: std::vec::Vec<u8> = std::vec::Vec::new();
    let mut vec_0_ref_0: &std::vec::Vec<u8> = &mut vec_0;
    let mut bytesmut_3: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::default();
    let mut bytesmut_3_ref_0: &crate::bytes_mut::BytesMut = &mut bytesmut_3;
    let mut bool_0: bool = crate::bytes_mut::BytesMut::eq(bytesmut_3_ref_0, vec_0_ref_0);
    let mut isize_4: &isize = crate::buf::chain::Chain::first_ref(chain_1_ref_0);
    let mut bytes_1: crate::bytes::Bytes = crate::bytes_mut::BytesMut::copy_to_bytes(bytesmut_2_ref_0, usize_3);
    let mut isize_5: &mut isize = crate::buf::chain::Chain::last_mut(chain_0_ref_0);
    crate::bytes_mut::BytesMut::advance(bytesmut_1_ref_0, usize_1);
    let mut usize_5: usize = crate::bytes_mut::BytesMut::remaining(bytesmut_0_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_48() {
    rusty_monitor::set_test_id(48);
    let mut str_0: &str = "Oizy06RSkA";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut usize_0: usize = 8604usize;
    let mut bytesmut_0: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::with_capacity(usize_0);
    let mut bytesmut_0_ref_0: &crate::bytes_mut::BytesMut = &mut bytesmut_0;
    let mut isize_0: isize = 1589isize;
    let mut isize_1: isize = -8794isize;
    let mut chain_0: crate::buf::chain::Chain<isize, isize> = crate::buf::chain::Chain {a: isize_1, b: isize_0};
    let mut bytes_0: crate::bytes::Bytes = crate::bytes::Bytes::default();
    let mut bytes_0_ref_0: &crate::bytes::Bytes = &mut bytes_0;
    let mut usize_1: usize = 9409usize;
    let mut bytes_1: crate::bytes::Bytes = crate::bytes::Bytes::new();
    let mut bytes_1_ref_0: &crate::bytes::Bytes = &mut bytes_1;
    let mut isize_2: isize = 11517isize;
    let mut intoiter_0: crate::buf::iter::IntoIter<isize> = crate::buf::iter::IntoIter::new(isize_2);
    let mut isize_3: isize = -4998isize;
    let mut isize_4: isize = 6094isize;
    let mut chain_1: crate::buf::chain::Chain<isize, isize> = crate::buf::chain::Chain::new(isize_4, isize_3);
    let mut usize_2: usize = 6123usize;
    let mut bytesmut_1: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::with_capacity(usize_2);
    let mut bytesmut_1_ref_0: &crate::bytes_mut::BytesMut = &mut bytesmut_1;
    let mut vec_0: std::vec::Vec<u8> = std::vec::Vec::new();
    let mut bytes_2: crate::bytes::Bytes = crate::bytes::Bytes::from(vec_0);
    let mut bytes_2_ref_0: &crate::bytes::Bytes = &mut bytes_2;
    let mut bool_0: bool = crate::bytes::Bytes::eq(bytes_2_ref_0, bytesmut_1_ref_0);
    let mut tuple_0: (isize, isize) = crate::buf::chain::Chain::into_inner(chain_1);
    let mut isize_5: isize = crate::buf::iter::IntoIter::into_inner(intoiter_0);
    let mut u8_slice_0: &[u8] = crate::bytes::Bytes::deref(bytes_1_ref_0);
    let mut u8_slice_1: &[u8] = crate::bytes::Bytes::deref(bytes_0_ref_0);
    let mut tuple_1: (isize, isize) = crate::buf::chain::Chain::into_inner(chain_0);
    let mut option_0: std::option::Option<std::cmp::Ordering> = crate::bytes_mut::BytesMut::partial_cmp(bytesmut_0_ref_0, str_0_ref_0);
    panic!("From RustyUnit with love");
}
}