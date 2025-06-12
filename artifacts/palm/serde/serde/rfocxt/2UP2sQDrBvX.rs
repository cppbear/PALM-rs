use crate::lib::*;
use crate::ser::{Error, Serialize, SerializeTuple, Serializer};
#[cfg(any(feature = "std", not(no_core_net)))]
const DEC_DIGITS_LUT: &[u8] = b"\
      0001020304050607080910111213141516171819\
      2021222324252627282930313233343536373839\
      4041424344454647484950515253545556575859\
      6061626364656667686970717273747576777879\
      8081828384858687888990919293949596979899";
macro_rules! primitive_impl {
    ($ty:ident, $method:ident $($cast:tt)*) => {
        impl Serialize for $ty { #[inline] fn serialize < S > (& self, serializer : S) ->
        Result < S::Ok, S::Error > where S : Serializer, { serializer.$method (* self
        $($cast)*) } }
    };
}
primitive_impl!(bool, serialize_bool);
primitive_impl!(isize, serialize_i64 as i64);
primitive_impl!(i8, serialize_i8);
primitive_impl!(i16, serialize_i16);
primitive_impl!(i32, serialize_i32);
primitive_impl!(i64, serialize_i64);
primitive_impl!(i128, serialize_i128);
primitive_impl!(usize, serialize_u64 as u64);
primitive_impl!(u8, serialize_u8);
primitive_impl!(u16, serialize_u16);
primitive_impl!(u32, serialize_u32);
primitive_impl!(u64, serialize_u64);
primitive_impl!(u128, serialize_u128);
primitive_impl!(f32, serialize_f32);
primitive_impl!(f64, serialize_f64);
primitive_impl!(char, serialize_char);
macro_rules! array_impls {
    ($($len:tt)+) => {
        $(impl < T > Serialize for [T; $len] where T : Serialize, { #[inline] fn
        serialize < S > (& self, serializer : S) -> Result < S::Ok, S::Error > where S :
        Serializer, { let mut seq = tri!(serializer.serialize_tuple($len)); for e in self
        { tri!(seq.serialize_element(e)); } seq.end() } })+
    };
}
array_impls! {
    01 02 03 04 05 06 07 08 09 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28
    29 30 31 32
}
macro_rules! seq_impl {
    ($(#[$attr:meta])* $ty:ident < T $(, $typaram:ident : $bound:ident)*>) => {
        $(#[$attr])* impl < T $(, $typaram)*> Serialize for $ty < T $(, $typaram)*> where
        T : Serialize, { #[inline] fn serialize < S > (& self, serializer : S) -> Result
        < S::Ok, S::Error > where S : Serializer, { serializer.collect_seq(self) } }
    };
}
seq_impl! {
    #[cfg(any(feature = "std", feature = "alloc"))] #[cfg_attr(docsrs,
    doc(cfg(any(feature = "std", feature = "alloc"))))] BinaryHeap < T >
}
seq_impl! {
    #[cfg(any(feature = "std", feature = "alloc"))] #[cfg_attr(docsrs,
    doc(cfg(any(feature = "std", feature = "alloc"))))] BTreeSet < T >
}
seq_impl! {
    #[cfg(feature = "std")] #[cfg_attr(docsrs, doc(cfg(feature = "std")))] HashSet < T, H
    : BuildHasher >
}
seq_impl! {
    #[cfg(any(feature = "std", feature = "alloc"))] #[cfg_attr(docsrs,
    doc(cfg(any(feature = "std", feature = "alloc"))))] LinkedList < T >
}
seq_impl! {
    #[cfg(any(feature = "std", feature = "alloc"))] #[cfg_attr(docsrs,
    doc(cfg(any(feature = "std", feature = "alloc"))))] Vec < T >
}
seq_impl! {
    #[cfg(any(feature = "std", feature = "alloc"))] #[cfg_attr(docsrs,
    doc(cfg(any(feature = "std", feature = "alloc"))))] VecDeque < T >
}
macro_rules! tuple_impls {
    ($($len:expr => ($($n:tt $name:ident)+))+) => {
        $(#[cfg_attr(docsrs, doc(hidden))] impl <$($name),+> Serialize for ($($name,)+)
        where $($name : Serialize,)+ { tuple_impl_body!($len => ($($n)+)); })+
    };
}
macro_rules! tuple_impl_body {
    ($len:expr => ($($n:tt)+)) => {
        #[inline] fn serialize < S > (& self, serializer : S) -> Result < S::Ok, S::Error
        > where S : Serializer, { let mut tuple = tri!(serializer.serialize_tuple($len));
        $(tri!(tuple.serialize_element(& self.$n));)+ tuple.end() }
    };
}
tuple_impls! {
    2 => (0 T0 1 T1) 3 => (0 T0 1 T1 2 T2) 4 => (0 T0 1 T1 2 T2 3 T3) 5 => (0 T0 1 T1 2
    T2 3 T3 4 T4) 6 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5) 7 => (0 T0 1 T1 2 T2 3 T3 4 T4 5
    T5 6 T6) 8 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7) 9 => (0 T0 1 T1 2 T2 3 T3 4
    T4 5 T5 6 T6 7 T7 8 T8) 10 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9) 11
    => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10) 12 => (0 T0 1 T1 2 T2 3
    T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11) 13 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5
    6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12) 14 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7
    T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13) 15 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7
    T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14) 16 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5
    6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14 15 T15)
}
macro_rules! map_impl {
    (
        $(#[$attr:meta])* $ty:ident < K $(: $kbound1:ident $(+ $kbound2:ident)*)*, V $(,
        $typaram:ident : $bound:ident)*>
    ) => {
        $(#[$attr])* impl < K, V $(, $typaram)*> Serialize for $ty < K, V $(, $typaram)*>
        where K : Serialize, V : Serialize, { #[inline] fn serialize < S > (& self,
        serializer : S) -> Result < S::Ok, S::Error > where S : Serializer, { serializer
        .collect_map(self) } }
    };
}
map_impl! {
    #[cfg(any(feature = "std", feature = "alloc"))] #[cfg_attr(docsrs,
    doc(cfg(any(feature = "std", feature = "alloc"))))] BTreeMap < K : Ord, V >
}
map_impl! {
    #[cfg(feature = "std")] #[cfg_attr(docsrs, doc(cfg(feature = "std")))] HashMap < K :
    Eq + Hash, V, H : BuildHasher >
}
macro_rules! deref_impl {
    ($(#[$attr:meta])* <$($desc:tt)+) => {
        $(#[$attr])* impl <$($desc)+ { #[inline] fn serialize < S > (& self, serializer :
        S) -> Result < S::Ok, S::Error > where S : Serializer, { (** self)
        .serialize(serializer) } }
    };
}
deref_impl! {
    <'a, T > Serialize for &'a T where T : ? Sized + Serialize
}
deref_impl! {
    <'a, T > Serialize for &'a mut T where T : ? Sized + Serialize
}
deref_impl! {
    #[cfg(any(feature = "std", feature = "alloc"))] #[cfg_attr(docsrs,
    doc(cfg(any(feature = "std", feature = "alloc"))))] < T > Serialize for Box < T >
    where T : ? Sized + Serialize
}
deref_impl! {
    #[doc = " This impl requires the [`\"rc\"`] Cargo feature of Serde."] #[doc = ""]
    #[doc = " Serializing a data structure containing `Rc` will serialize a copy of"]
    #[doc = " the contents of the `Rc` each time the `Rc` is referenced within the"]
    #[doc = " data structure. Serialization will not attempt to deduplicate these"] #[doc
    = " repeated data."] #[doc = ""] #[doc =
    " [`\"rc\"`]: https://serde.rs/feature-flags.html#-features-rc"] #[cfg(all(feature =
    "rc", any(feature = "std", feature = "alloc")))] #[cfg_attr(docsrs,
    doc(cfg(all(feature = "rc", any(feature = "std", feature = "alloc")))))] < T >
    Serialize for Rc < T > where T : ? Sized + Serialize
}
deref_impl! {
    #[doc = " This impl requires the [`\"rc\"`] Cargo feature of Serde."] #[doc = ""]
    #[doc = " Serializing a data structure containing `Arc` will serialize a copy of"]
    #[doc = " the contents of the `Arc` each time the `Arc` is referenced within the"]
    #[doc = " data structure. Serialization will not attempt to deduplicate these"] #[doc
    = " repeated data."] #[doc = ""] #[doc =
    " [`\"rc\"`]: https://serde.rs/feature-flags.html#-features-rc"] #[cfg(all(feature =
    "rc", any(feature = "std", feature = "alloc")))] #[cfg_attr(docsrs,
    doc(cfg(all(feature = "rc", any(feature = "std", feature = "alloc")))))] < T >
    Serialize for Arc < T > where T : ? Sized + Serialize
}
deref_impl! {
    #[cfg(any(feature = "std", feature = "alloc"))] #[cfg_attr(docsrs,
    doc(cfg(any(feature = "std", feature = "alloc"))))] <'a, T > Serialize for Cow <'a, T
    > where T : ? Sized + Serialize + ToOwned
}
macro_rules! nonzero_integers {
    ($($T:ident,)+) => {
        $(impl Serialize for num::$T { fn serialize < S > (& self, serializer : S) ->
        Result < S::Ok, S::Error > where S : Serializer, { self.get()
        .serialize(serializer) } })+
    };
}
nonzero_integers! {
    NonZeroI8, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI128, NonZeroIsize, NonZeroU8,
    NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU128, NonZeroUsize,
}
#[cfg(any(feature = "std", not(no_core_net)))]
macro_rules! serialize_display_bounded_length {
    ($value:expr, $max:expr, $serializer:expr) => {
        { let mut buffer = [0u8; $max]; let mut writer = crate ::format::Buf::new(& mut
        buffer); write!(& mut writer, "{}", $value) .unwrap(); $serializer
        .serialize_str(writer.as_str()) }
    };
}
#[cfg(all(feature = "std", not(no_std_atomic)))]
macro_rules! atomic_impl {
    ($($ty:ident $size:expr)*) => {
        $(#[cfg(any(no_target_has_atomic, target_has_atomic = $size))] #[cfg_attr(docsrs,
        doc(cfg(all(feature = "std", target_has_atomic = $size))))] impl Serialize for
        $ty { fn serialize < S > (& self, serializer : S) -> Result < S::Ok, S::Error >
        where S : Serializer, { self.load(Ordering::Relaxed).serialize(serializer) } })*
    };
}
#[cfg(all(feature = "std", not(no_std_atomic)))]
atomic_impl! {
    AtomicBool "8" AtomicI8 "8" AtomicI16 "16" AtomicI32 "32" AtomicIsize "ptr" AtomicU8
    "8" AtomicU16 "16" AtomicU32 "32" AtomicUsize "ptr"
}
#[cfg(all(feature = "std", not(no_std_atomic64)))]
atomic_impl! {
    AtomicI64 "64" AtomicU64 "64"
}
#[cfg_attr(
    not(no_diagnostic_namespace),
    diagnostic::on_unimplemented(
        note = "for local types consider adding `#[derive(serde::Serialize)]` to your `{Self}` type",
        note = "for types from other crates check whether the crate offers a `serde` feature flag",
    )
)]
pub trait Serialize {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer;
}
pub(super) struct Buf<'a> {
    bytes: &'a mut [u8],
    offset: usize,
}
#[cfg(any(feature = "std", not(no_core_net)))]
impl Serialize for net::SocketAddrV4 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if serializer.is_human_readable() {
            const MAX_LEN: usize = 21;
            debug_assert_eq!(MAX_LEN, "101.102.103.104:65000".len());
            serialize_display_bounded_length!(self, MAX_LEN, serializer)
        } else {
            (self.ip(), self.port()).serialize(serializer)
        }
    }
}
impl<'a> Buf<'a> {
    pub fn new(bytes: &'a mut [u8]) -> Self {
        Buf { bytes, offset: 0 }
    }
    pub fn as_str(&self) -> &str {
        let slice = &self.bytes[..self.offset];
        unsafe { str::from_utf8_unchecked(slice) }
    }
}
