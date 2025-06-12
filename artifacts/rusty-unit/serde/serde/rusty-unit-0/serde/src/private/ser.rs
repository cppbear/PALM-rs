use crate::lib::*;

use crate::ser::{self, Impossible, Serialize, SerializeMap, SerializeStruct, Serializer};

#[cfg(any(feature = "std", feature = "alloc"))]
use self::content::{
    Content, ContentSerializer, SerializeStructVariantAsMapValue, SerializeTupleVariantAsMapValue,
};

/// Used to check that serde(getter) attributes return the expected type.
/// Not public API.
pub fn constrain<T: ?Sized>(t: &T) -> &T {
    t
}

/// Not public API.
pub fn serialize_tagged_newtype<S, T>(
    serializer: S,
    type_ident: &'static str,
    variant_ident: &'static str,
    tag: &'static str,
    variant_name: &'static str,
    value: &T,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: Serialize,
{
    value.serialize(TaggedSerializer {
        type_ident,
        variant_ident,
        tag,
        variant_name,
        delegate: serializer,
    })
}

struct TaggedSerializer<S> {
    type_ident: &'static str,
    variant_ident: &'static str,
    tag: &'static str,
    variant_name: &'static str,
    delegate: S,
}

enum Unsupported {
    Boolean,
    Integer,
    Float,
    Char,
    String,
    ByteArray,
    Optional,
    Sequence,
    Tuple,
    TupleStruct,
    #[cfg(not(any(feature = "std", feature = "alloc")))]
    Enum,
}

impl Display for Unsupported {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Unsupported::Boolean => formatter.write_str("a boolean"),
            Unsupported::Integer => formatter.write_str("an integer"),
            Unsupported::Float => formatter.write_str("a float"),
            Unsupported::Char => formatter.write_str("a char"),
            Unsupported::String => formatter.write_str("a string"),
            Unsupported::ByteArray => formatter.write_str("a byte array"),
            Unsupported::Optional => formatter.write_str("an optional"),
            Unsupported::Sequence => formatter.write_str("a sequence"),
            Unsupported::Tuple => formatter.write_str("a tuple"),
            Unsupported::TupleStruct => formatter.write_str("a tuple struct"),
            #[cfg(not(any(feature = "std", feature = "alloc")))]
            Unsupported::Enum => formatter.write_str("an enum"),
        }
    }
}

impl<S> TaggedSerializer<S>
where
    S: Serializer,
{
    fn bad_type(self, what: Unsupported) -> S::Error {
        ser::Error::custom(format_args!(
            "cannot serialize tagged newtype variant {}::{} containing {}",
            self.type_ident, self.variant_ident, what
        ))
    }
}

impl<S> Serializer for TaggedSerializer<S>
where
    S: Serializer,
{
    type Ok = S::Ok;
    type Error = S::Error;

    type SerializeSeq = Impossible<S::Ok, S::Error>;
    type SerializeTuple = Impossible<S::Ok, S::Error>;
    type SerializeTupleStruct = Impossible<S::Ok, S::Error>;
    type SerializeMap = S::SerializeMap;
    type SerializeStruct = S::SerializeStruct;

    #[cfg(not(any(feature = "std", feature = "alloc")))]
    type SerializeTupleVariant = Impossible<S::Ok, S::Error>;
    #[cfg(any(feature = "std", feature = "alloc"))]
    type SerializeTupleVariant = SerializeTupleVariantAsMapValue<S::SerializeMap>;

    #[cfg(not(any(feature = "std", feature = "alloc")))]
    type SerializeStructVariant = Impossible<S::Ok, S::Error>;
    #[cfg(any(feature = "std", feature = "alloc"))]
    type SerializeStructVariant = SerializeStructVariantAsMapValue<S::SerializeMap>;

    fn serialize_bool(self, _: bool) -> Result<Self::Ok, Self::Error> {
        Err(self.bad_type(Unsupported::Boolean))
    }

    fn serialize_i8(self, _: i8) -> Result<Self::Ok, Self::Error> {
        Err(self.bad_type(Unsupported::Integer))
    }

    fn serialize_i16(self, _: i16) -> Result<Self::Ok, Self::Error> {
        Err(self.bad_type(Unsupported::Integer))
    }

    fn serialize_i32(self, _: i32) -> Result<Self::Ok, Self::Error> {
        Err(self.bad_type(Unsupported::Integer))
    }

    fn serialize_i64(self, _: i64) -> Result<Self::Ok, Self::Error> {
        Err(self.bad_type(Unsupported::Integer))
    }

    fn serialize_u8(self, _: u8) -> Result<Self::Ok, Self::Error> {
        Err(self.bad_type(Unsupported::Integer))
    }

    fn serialize_u16(self, _: u16) -> Result<Self::Ok, Self::Error> {
        Err(self.bad_type(Unsupported::Integer))
    }

    fn serialize_u32(self, _: u32) -> Result<Self::Ok, Self::Error> {
        Err(self.bad_type(Unsupported::Integer))
    }

    fn serialize_u64(self, _: u64) -> Result<Self::Ok, Self::Error> {
        Err(self.bad_type(Unsupported::Integer))
    }

    fn serialize_f32(self, _: f32) -> Result<Self::Ok, Self::Error> {
        Err(self.bad_type(Unsupported::Float))
    }

    fn serialize_f64(self, _: f64) -> Result<Self::Ok, Self::Error> {
        Err(self.bad_type(Unsupported::Float))
    }

    fn serialize_char(self, _: char) -> Result<Self::Ok, Self::Error> {
        Err(self.bad_type(Unsupported::Char))
    }

    fn serialize_str(self, _: &str) -> Result<Self::Ok, Self::Error> {
        Err(self.bad_type(Unsupported::String))
    }

    fn serialize_bytes(self, _: &[u8]) -> Result<Self::Ok, Self::Error> {
        Err(self.bad_type(Unsupported::ByteArray))
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Err(self.bad_type(Unsupported::Optional))
    }

    fn serialize_some<T>(self, _: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        Err(self.bad_type(Unsupported::Optional))
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        let mut map = tri!(self.delegate.serialize_map(Some(1)));
        tri!(map.serialize_entry(self.tag, self.variant_name));
        map.end()
    }

    fn serialize_unit_struct(self, _: &'static str) -> Result<Self::Ok, Self::Error> {
        let mut map = tri!(self.delegate.serialize_map(Some(1)));
        tri!(map.serialize_entry(self.tag, self.variant_name));
        map.end()
    }

    fn serialize_unit_variant(
        self,
        _: &'static str,
        _: u32,
        inner_variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        let mut map = tri!(self.delegate.serialize_map(Some(2)));
        tri!(map.serialize_entry(self.tag, self.variant_name));
        tri!(map.serialize_entry(inner_variant, &()));
        map.end()
    }

    fn serialize_newtype_struct<T>(
        self,
        _: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T>(
        self,
        _: &'static str,
        _: u32,
        inner_variant: &'static str,
        inner_value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        let mut map = tri!(self.delegate.serialize_map(Some(2)));
        tri!(map.serialize_entry(self.tag, self.variant_name));
        tri!(map.serialize_entry(inner_variant, inner_value));
        map.end()
    }

    fn serialize_seq(self, _: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Err(self.bad_type(Unsupported::Sequence))
    }

    fn serialize_tuple(self, _: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Err(self.bad_type(Unsupported::Tuple))
    }

    fn serialize_tuple_struct(
        self,
        _: &'static str,
        _: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Err(self.bad_type(Unsupported::TupleStruct))
    }

    #[cfg(not(any(feature = "std", feature = "alloc")))]
    fn serialize_tuple_variant(
        self,
        _: &'static str,
        _: u32,
        _: &'static str,
        _: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        // Lack of push-based serialization means we need to buffer the content
        // of the tuple variant, so it requires std.
        Err(self.bad_type(Unsupported::Enum))
    }

    #[cfg(any(feature = "std", feature = "alloc"))]
    fn serialize_tuple_variant(
        self,
        _: &'static str,
        _: u32,
        inner_variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        let mut map = tri!(self.delegate.serialize_map(Some(2)));
        tri!(map.serialize_entry(self.tag, self.variant_name));
        tri!(map.serialize_key(inner_variant));
        Ok(SerializeTupleVariantAsMapValue::new(
            map,
            inner_variant,
            len,
        ))
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        let mut map = tri!(self.delegate.serialize_map(len.map(|len| len + 1)));
        tri!(map.serialize_entry(self.tag, self.variant_name));
        Ok(map)
    }

    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        let mut state = tri!(self.delegate.serialize_struct(name, len + 1));
        tri!(state.serialize_field(self.tag, self.variant_name));
        Ok(state)
    }

    #[cfg(not(any(feature = "std", feature = "alloc")))]
    fn serialize_struct_variant(
        self,
        _: &'static str,
        _: u32,
        _: &'static str,
        _: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        // Lack of push-based serialization means we need to buffer the content
        // of the struct variant, so it requires std.
        Err(self.bad_type(Unsupported::Enum))
    }

    #[cfg(any(feature = "std", feature = "alloc"))]
    fn serialize_struct_variant(
        self,
        _: &'static str,
        _: u32,
        inner_variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        let mut map = tri!(self.delegate.serialize_map(Some(2)));
        tri!(map.serialize_entry(self.tag, self.variant_name));
        tri!(map.serialize_key(inner_variant));
        Ok(SerializeStructVariantAsMapValue::new(
            map,
            inner_variant,
            len,
        ))
    }

    #[cfg(not(any(feature = "std", feature = "alloc")))]
    fn collect_str<T>(self, _: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Display,
    {
        Err(self.bad_type(Unsupported::String))
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
pub mod content {
    use crate::lib::*;

    use crate::ser::{self, Serialize, Serializer};

    pub struct SerializeTupleVariantAsMapValue<M> {
        pub map: M,
        pub name: &'static str,
        pub fields: Vec<Content>,
    }

    impl<M> SerializeTupleVariantAsMapValue<M> {
        pub fn new(map: M, name: &'static str, len: usize) -> Self {
            SerializeTupleVariantAsMapValue {
                map,
                name,
                fields: Vec::with_capacity(len),
            }
        }
    }

    impl<M> ser::SerializeTupleVariant for SerializeTupleVariantAsMapValue<M>
    where
        M: ser::SerializeMap,
    {
        type Ok = M::Ok;
        type Error = M::Error;

        fn serialize_field<T>(&mut self, value: &T) -> Result<(), M::Error>
        where
            T: ?Sized + Serialize,
        {
            let value = tri!(value.serialize(ContentSerializer::<M::Error>::new()));
            self.fields.push(value);
            Ok(())
        }

        fn end(mut self) -> Result<M::Ok, M::Error> {
            tri!(self
                .map
                .serialize_value(&Content::TupleStruct(self.name, self.fields)));
            self.map.end()
        }
    }

    pub struct SerializeStructVariantAsMapValue<M> {
        pub map: M,
        pub name: &'static str,
        pub fields: Vec<(&'static str, Content)>,
    }

    impl<M> SerializeStructVariantAsMapValue<M> {
        pub fn new(map: M, name: &'static str, len: usize) -> Self {
            SerializeStructVariantAsMapValue {
                map,
                name,
                fields: Vec::with_capacity(len),
            }
        }
    }

    impl<M> ser::SerializeStructVariant for SerializeStructVariantAsMapValue<M>
    where
        M: ser::SerializeMap,
    {
        type Ok = M::Ok;
        type Error = M::Error;

        fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), M::Error>
        where
            T: ?Sized + Serialize,
        {
            let value = tri!(value.serialize(ContentSerializer::<M::Error>::new()));
            self.fields.push((key, value));
            Ok(())
        }

        fn end(mut self) -> Result<M::Ok, M::Error> {
            tri!(self
                .map
                .serialize_value(&Content::Struct(self.name, self.fields)));
            self.map.end()
        }
    }

    pub enum Content {
        Bool(bool),

        U8(u8),
        U16(u16),
        U32(u32),
        U64(u64),

        I8(i8),
        I16(i16),
        I32(i32),
        I64(i64),

        F32(f32),
        F64(f64),

        Char(char),
        String(String),
        Bytes(Vec<u8>),

        None,
        Some(Box<Content>),

        Unit,
        UnitStruct(&'static str),
        UnitVariant(&'static str, u32, &'static str),
        NewtypeStruct(&'static str, Box<Content>),
        NewtypeVariant(&'static str, u32, &'static str, Box<Content>),

        Seq(Vec<Content>),
        Tuple(Vec<Content>),
        TupleStruct(&'static str, Vec<Content>),
        TupleVariant(&'static str, u32, &'static str, Vec<Content>),
        Map(Vec<(Content, Content)>),
        Struct(&'static str, Vec<(&'static str, Content)>),
        StructVariant(
            &'static str,
            u32,
            &'static str,
            Vec<(&'static str, Content)>,
        ),
    }

    impl Serialize for Content {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match *self {
                Content::Bool(b) => serializer.serialize_bool(b),
                Content::U8(u) => serializer.serialize_u8(u),
                Content::U16(u) => serializer.serialize_u16(u),
                Content::U32(u) => serializer.serialize_u32(u),
                Content::U64(u) => serializer.serialize_u64(u),
                Content::I8(i) => serializer.serialize_i8(i),
                Content::I16(i) => serializer.serialize_i16(i),
                Content::I32(i) => serializer.serialize_i32(i),
                Content::I64(i) => serializer.serialize_i64(i),
                Content::F32(f) => serializer.serialize_f32(f),
                Content::F64(f) => serializer.serialize_f64(f),
                Content::Char(c) => serializer.serialize_char(c),
                Content::String(ref s) => serializer.serialize_str(s),
                Content::Bytes(ref b) => serializer.serialize_bytes(b),
                Content::None => serializer.serialize_none(),
                Content::Some(ref c) => serializer.serialize_some(&**c),
                Content::Unit => serializer.serialize_unit(),
                Content::UnitStruct(n) => serializer.serialize_unit_struct(n),
                Content::UnitVariant(n, i, v) => serializer.serialize_unit_variant(n, i, v),
                Content::NewtypeStruct(n, ref c) => serializer.serialize_newtype_struct(n, &**c),
                Content::NewtypeVariant(n, i, v, ref c) => {
                    serializer.serialize_newtype_variant(n, i, v, &**c)
                }
                Content::Seq(ref elements) => elements.serialize(serializer),
                Content::Tuple(ref elements) => {
                    use crate::ser::SerializeTuple;
                    let mut tuple = tri!(serializer.serialize_tuple(elements.len()));
                    for e in elements {
                        tri!(tuple.serialize_element(e));
                    }
                    tuple.end()
                }
                Content::TupleStruct(n, ref fields) => {
                    use crate::ser::SerializeTupleStruct;
                    let mut ts = tri!(serializer.serialize_tuple_struct(n, fields.len()));
                    for f in fields {
                        tri!(ts.serialize_field(f));
                    }
                    ts.end()
                }
                Content::TupleVariant(n, i, v, ref fields) => {
                    use crate::ser::SerializeTupleVariant;
                    let mut tv = tri!(serializer.serialize_tuple_variant(n, i, v, fields.len()));
                    for f in fields {
                        tri!(tv.serialize_field(f));
                    }
                    tv.end()
                }
                Content::Map(ref entries) => {
                    use crate::ser::SerializeMap;
                    let mut map = tri!(serializer.serialize_map(Some(entries.len())));
                    for (k, v) in entries {
                        tri!(map.serialize_entry(k, v));
                    }
                    map.end()
                }
                Content::Struct(n, ref fields) => {
                    use crate::ser::SerializeStruct;
                    let mut s = tri!(serializer.serialize_struct(n, fields.len()));
                    for &(k, ref v) in fields {
                        tri!(s.serialize_field(k, v));
                    }
                    s.end()
                }
                Content::StructVariant(n, i, v, ref fields) => {
                    use crate::ser::SerializeStructVariant;
                    let mut sv = tri!(serializer.serialize_struct_variant(n, i, v, fields.len()));
                    for &(k, ref v) in fields {
                        tri!(sv.serialize_field(k, v));
                    }
                    sv.end()
                }
            }
        }
    }

    pub struct ContentSerializer<E> {
        error: PhantomData<E>,
    }

    impl<E> ContentSerializer<E> {
        pub fn new() -> Self {
            ContentSerializer { error: PhantomData }
        }
    }

    impl<E> Serializer for ContentSerializer<E>
    where
        E: ser::Error,
    {
        type Ok = Content;
        type Error = E;

        type SerializeSeq = SerializeSeq<E>;
        type SerializeTuple = SerializeTuple<E>;
        type SerializeTupleStruct = SerializeTupleStruct<E>;
        type SerializeTupleVariant = SerializeTupleVariant<E>;
        type SerializeMap = SerializeMap<E>;
        type SerializeStruct = SerializeStruct<E>;
        type SerializeStructVariant = SerializeStructVariant<E>;

        fn serialize_bool(self, v: bool) -> Result<Content, E> {
            Ok(Content::Bool(v))
        }

        fn serialize_i8(self, v: i8) -> Result<Content, E> {
            Ok(Content::I8(v))
        }

        fn serialize_i16(self, v: i16) -> Result<Content, E> {
            Ok(Content::I16(v))
        }

        fn serialize_i32(self, v: i32) -> Result<Content, E> {
            Ok(Content::I32(v))
        }

        fn serialize_i64(self, v: i64) -> Result<Content, E> {
            Ok(Content::I64(v))
        }

        fn serialize_u8(self, v: u8) -> Result<Content, E> {
            Ok(Content::U8(v))
        }

        fn serialize_u16(self, v: u16) -> Result<Content, E> {
            Ok(Content::U16(v))
        }

        fn serialize_u32(self, v: u32) -> Result<Content, E> {
            Ok(Content::U32(v))
        }

        fn serialize_u64(self, v: u64) -> Result<Content, E> {
            Ok(Content::U64(v))
        }

        fn serialize_f32(self, v: f32) -> Result<Content, E> {
            Ok(Content::F32(v))
        }

        fn serialize_f64(self, v: f64) -> Result<Content, E> {
            Ok(Content::F64(v))
        }

        fn serialize_char(self, v: char) -> Result<Content, E> {
            Ok(Content::Char(v))
        }

        fn serialize_str(self, value: &str) -> Result<Content, E> {
            Ok(Content::String(value.to_owned()))
        }

        fn serialize_bytes(self, value: &[u8]) -> Result<Content, E> {
            Ok(Content::Bytes(value.to_owned()))
        }

        fn serialize_none(self) -> Result<Content, E> {
            Ok(Content::None)
        }

        fn serialize_some<T>(self, value: &T) -> Result<Content, E>
        where
            T: ?Sized + Serialize,
        {
            Ok(Content::Some(Box::new(tri!(value.serialize(self)))))
        }

        fn serialize_unit(self) -> Result<Content, E> {
            Ok(Content::Unit)
        }

        fn serialize_unit_struct(self, name: &'static str) -> Result<Content, E> {
            Ok(Content::UnitStruct(name))
        }

        fn serialize_unit_variant(
            self,
            name: &'static str,
            variant_index: u32,
            variant: &'static str,
        ) -> Result<Content, E> {
            Ok(Content::UnitVariant(name, variant_index, variant))
        }

        fn serialize_newtype_struct<T>(self, name: &'static str, value: &T) -> Result<Content, E>
        where
            T: ?Sized + Serialize,
        {
            Ok(Content::NewtypeStruct(
                name,
                Box::new(tri!(value.serialize(self))),
            ))
        }

        fn serialize_newtype_variant<T>(
            self,
            name: &'static str,
            variant_index: u32,
            variant: &'static str,
            value: &T,
        ) -> Result<Content, E>
        where
            T: ?Sized + Serialize,
        {
            Ok(Content::NewtypeVariant(
                name,
                variant_index,
                variant,
                Box::new(tri!(value.serialize(self))),
            ))
        }

        fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, E> {
            Ok(SerializeSeq {
                elements: Vec::with_capacity(len.unwrap_or(0)),
                error: PhantomData,
            })
        }

        fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, E> {
            Ok(SerializeTuple {
                elements: Vec::with_capacity(len),
                error: PhantomData,
            })
        }

        fn serialize_tuple_struct(
            self,
            name: &'static str,
            len: usize,
        ) -> Result<Self::SerializeTupleStruct, E> {
            Ok(SerializeTupleStruct {
                name,
                fields: Vec::with_capacity(len),
                error: PhantomData,
            })
        }

        fn serialize_tuple_variant(
            self,
            name: &'static str,
            variant_index: u32,
            variant: &'static str,
            len: usize,
        ) -> Result<Self::SerializeTupleVariant, E> {
            Ok(SerializeTupleVariant {
                name,
                variant_index,
                variant,
                fields: Vec::with_capacity(len),
                error: PhantomData,
            })
        }

        fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, E> {
            Ok(SerializeMap {
                entries: Vec::with_capacity(len.unwrap_or(0)),
                key: None,
                error: PhantomData,
            })
        }

        fn serialize_struct(
            self,
            name: &'static str,
            len: usize,
        ) -> Result<Self::SerializeStruct, E> {
            Ok(SerializeStruct {
                name,
                fields: Vec::with_capacity(len),
                error: PhantomData,
            })
        }

        fn serialize_struct_variant(
            self,
            name: &'static str,
            variant_index: u32,
            variant: &'static str,
            len: usize,
        ) -> Result<Self::SerializeStructVariant, E> {
            Ok(SerializeStructVariant {
                name,
                variant_index,
                variant,
                fields: Vec::with_capacity(len),
                error: PhantomData,
            })
        }
    }

    pub struct SerializeSeq<E> {
        elements: Vec<Content>,
        error: PhantomData<E>,
    }

    impl<E> ser::SerializeSeq for SerializeSeq<E>
    where
        E: ser::Error,
    {
        type Ok = Content;
        type Error = E;

        fn serialize_element<T>(&mut self, value: &T) -> Result<(), E>
        where
            T: ?Sized + Serialize,
        {
            let value = tri!(value.serialize(ContentSerializer::<E>::new()));
            self.elements.push(value);
            Ok(())
        }

        fn end(self) -> Result<Content, E> {
            Ok(Content::Seq(self.elements))
        }
    }

    pub struct SerializeTuple<E> {
        elements: Vec<Content>,
        error: PhantomData<E>,
    }

    impl<E> ser::SerializeTuple for SerializeTuple<E>
    where
        E: ser::Error,
    {
        type Ok = Content;
        type Error = E;

        fn serialize_element<T>(&mut self, value: &T) -> Result<(), E>
        where
            T: ?Sized + Serialize,
        {
            let value = tri!(value.serialize(ContentSerializer::<E>::new()));
            self.elements.push(value);
            Ok(())
        }

        fn end(self) -> Result<Content, E> {
            Ok(Content::Tuple(self.elements))
        }
    }

    pub struct SerializeTupleStruct<E> {
        name: &'static str,
        fields: Vec<Content>,
        error: PhantomData<E>,
    }

    impl<E> ser::SerializeTupleStruct for SerializeTupleStruct<E>
    where
        E: ser::Error,
    {
        type Ok = Content;
        type Error = E;

        fn serialize_field<T>(&mut self, value: &T) -> Result<(), E>
        where
            T: ?Sized + Serialize,
        {
            let value = tri!(value.serialize(ContentSerializer::<E>::new()));
            self.fields.push(value);
            Ok(())
        }

        fn end(self) -> Result<Content, E> {
            Ok(Content::TupleStruct(self.name, self.fields))
        }
    }

    pub struct SerializeTupleVariant<E> {
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        fields: Vec<Content>,
        error: PhantomData<E>,
    }

    impl<E> ser::SerializeTupleVariant for SerializeTupleVariant<E>
    where
        E: ser::Error,
    {
        type Ok = Content;
        type Error = E;

        fn serialize_field<T>(&mut self, value: &T) -> Result<(), E>
        where
            T: ?Sized + Serialize,
        {
            let value = tri!(value.serialize(ContentSerializer::<E>::new()));
            self.fields.push(value);
            Ok(())
        }

        fn end(self) -> Result<Content, E> {
            Ok(Content::TupleVariant(
                self.name,
                self.variant_index,
                self.variant,
                self.fields,
            ))
        }
    }

    pub struct SerializeMap<E> {
        entries: Vec<(Content, Content)>,
        key: Option<Content>,
        error: PhantomData<E>,
    }

    impl<E> ser::SerializeMap for SerializeMap<E>
    where
        E: ser::Error,
    {
        type Ok = Content;
        type Error = E;

        fn serialize_key<T>(&mut self, key: &T) -> Result<(), E>
        where
            T: ?Sized + Serialize,
        {
            let key = tri!(key.serialize(ContentSerializer::<E>::new()));
            self.key = Some(key);
            Ok(())
        }

        fn serialize_value<T>(&mut self, value: &T) -> Result<(), E>
        where
            T: ?Sized + Serialize,
        {
            let key = self
                .key
                .take()
                .expect("serialize_value called before serialize_key");
            let value = tri!(value.serialize(ContentSerializer::<E>::new()));
            self.entries.push((key, value));
            Ok(())
        }

        fn end(self) -> Result<Content, E> {
            Ok(Content::Map(self.entries))
        }

        fn serialize_entry<K, V>(&mut self, key: &K, value: &V) -> Result<(), E>
        where
            K: ?Sized + Serialize,
            V: ?Sized + Serialize,
        {
            let key = tri!(key.serialize(ContentSerializer::<E>::new()));
            let value = tri!(value.serialize(ContentSerializer::<E>::new()));
            self.entries.push((key, value));
            Ok(())
        }
    }

    pub struct SerializeStruct<E> {
        name: &'static str,
        fields: Vec<(&'static str, Content)>,
        error: PhantomData<E>,
    }

    impl<E> ser::SerializeStruct for SerializeStruct<E>
    where
        E: ser::Error,
    {
        type Ok = Content;
        type Error = E;

        fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), E>
        where
            T: ?Sized + Serialize,
        {
            let value = tri!(value.serialize(ContentSerializer::<E>::new()));
            self.fields.push((key, value));
            Ok(())
        }

        fn end(self) -> Result<Content, E> {
            Ok(Content::Struct(self.name, self.fields))
        }
    }

    pub struct SerializeStructVariant<E> {
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        fields: Vec<(&'static str, Content)>,
        error: PhantomData<E>,
    }

    impl<E> ser::SerializeStructVariant for SerializeStructVariant<E>
    where
        E: ser::Error,
    {
        type Ok = Content;
        type Error = E;

        fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), E>
        where
            T: ?Sized + Serialize,
        {
            let value = tri!(value.serialize(ContentSerializer::<E>::new()));
            self.fields.push((key, value));
            Ok(())
        }

        fn end(self) -> Result<Content, E> {
            Ok(Content::StructVariant(
                self.name,
                self.variant_index,
                self.variant,
                self.fields,
            ))
        }
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
pub struct FlatMapSerializer<'a, M: 'a>(pub &'a mut M);

#[cfg(any(feature = "std", feature = "alloc"))]
impl<'a, M> FlatMapSerializer<'a, M>
where
    M: SerializeMap + 'a,
{
    fn bad_type(what: Unsupported) -> M::Error {
        ser::Error::custom(format_args!(
            "can only flatten structs and maps (got {})",
            what
        ))
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl<'a, M> Serializer for FlatMapSerializer<'a, M>
where
    M: SerializeMap + 'a,
{
    type Ok = ();
    type Error = M::Error;

    type SerializeSeq = Impossible<Self::Ok, M::Error>;
    type SerializeTuple = Impossible<Self::Ok, M::Error>;
    type SerializeTupleStruct = Impossible<Self::Ok, M::Error>;
    type SerializeMap = FlatMapSerializeMap<'a, M>;
    type SerializeStruct = FlatMapSerializeStruct<'a, M>;
    type SerializeTupleVariant = FlatMapSerializeTupleVariantAsMapValue<'a, M>;
    type SerializeStructVariant = FlatMapSerializeStructVariantAsMapValue<'a, M>;

    fn serialize_bool(self, _: bool) -> Result<Self::Ok, Self::Error> {
        Err(Self::bad_type(Unsupported::Boolean))
    }

    fn serialize_i8(self, _: i8) -> Result<Self::Ok, Self::Error> {
        Err(Self::bad_type(Unsupported::Integer))
    }

    fn serialize_i16(self, _: i16) -> Result<Self::Ok, Self::Error> {
        Err(Self::bad_type(Unsupported::Integer))
    }

    fn serialize_i32(self, _: i32) -> Result<Self::Ok, Self::Error> {
        Err(Self::bad_type(Unsupported::Integer))
    }

    fn serialize_i64(self, _: i64) -> Result<Self::Ok, Self::Error> {
        Err(Self::bad_type(Unsupported::Integer))
    }

    fn serialize_u8(self, _: u8) -> Result<Self::Ok, Self::Error> {
        Err(Self::bad_type(Unsupported::Integer))
    }

    fn serialize_u16(self, _: u16) -> Result<Self::Ok, Self::Error> {
        Err(Self::bad_type(Unsupported::Integer))
    }

    fn serialize_u32(self, _: u32) -> Result<Self::Ok, Self::Error> {
        Err(Self::bad_type(Unsupported::Integer))
    }

    fn serialize_u64(self, _: u64) -> Result<Self::Ok, Self::Error> {
        Err(Self::bad_type(Unsupported::Integer))
    }

    fn serialize_f32(self, _: f32) -> Result<Self::Ok, Self::Error> {
        Err(Self::bad_type(Unsupported::Float))
    }

    fn serialize_f64(self, _: f64) -> Result<Self::Ok, Self::Error> {
        Err(Self::bad_type(Unsupported::Float))
    }

    fn serialize_char(self, _: char) -> Result<Self::Ok, Self::Error> {
        Err(Self::bad_type(Unsupported::Char))
    }

    fn serialize_str(self, _: &str) -> Result<Self::Ok, Self::Error> {
        Err(Self::bad_type(Unsupported::String))
    }

    fn serialize_bytes(self, _: &[u8]) -> Result<Self::Ok, Self::Error> {
        Err(Self::bad_type(Unsupported::ByteArray))
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }

    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }

    fn serialize_unit_struct(self, _: &'static str) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }

    fn serialize_unit_variant(
        self,
        _: &'static str,
        _: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        self.0.serialize_entry(variant, &())
    }

    fn serialize_newtype_struct<T>(
        self,
        _: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T>(
        self,
        _: &'static str,
        _: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.0.serialize_entry(variant, value)
    }

    fn serialize_seq(self, _: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Err(Self::bad_type(Unsupported::Sequence))
    }

    fn serialize_tuple(self, _: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Err(Self::bad_type(Unsupported::Tuple))
    }

    fn serialize_tuple_struct(
        self,
        _: &'static str,
        _: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Err(Self::bad_type(Unsupported::TupleStruct))
    }

    fn serialize_tuple_variant(
        self,
        _: &'static str,
        _: u32,
        variant: &'static str,
        _: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        tri!(self.0.serialize_key(variant));
        Ok(FlatMapSerializeTupleVariantAsMapValue::new(self.0))
    }

    fn serialize_map(self, _: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Ok(FlatMapSerializeMap(self.0))
    }

    fn serialize_struct(
        self,
        _: &'static str,
        _: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(FlatMapSerializeStruct(self.0))
    }

    fn serialize_struct_variant(
        self,
        _: &'static str,
        _: u32,
        inner_variant: &'static str,
        _: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        tri!(self.0.serialize_key(inner_variant));
        Ok(FlatMapSerializeStructVariantAsMapValue::new(
            self.0,
            inner_variant,
        ))
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
pub struct FlatMapSerializeMap<'a, M: 'a>(&'a mut M);

#[cfg(any(feature = "std", feature = "alloc"))]
impl<'a, M> ser::SerializeMap for FlatMapSerializeMap<'a, M>
where
    M: SerializeMap + 'a,
{
    type Ok = ();
    type Error = M::Error;

    fn serialize_key<T>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.0.serialize_key(key)
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.0.serialize_value(value)
    }

    fn serialize_entry<K, V>(&mut self, key: &K, value: &V) -> Result<(), Self::Error>
    where
        K: ?Sized + Serialize,
        V: ?Sized + Serialize,
    {
        self.0.serialize_entry(key, value)
    }

    fn end(self) -> Result<(), Self::Error> {
        Ok(())
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
pub struct FlatMapSerializeStruct<'a, M: 'a>(&'a mut M);

#[cfg(any(feature = "std", feature = "alloc"))]
impl<'a, M> ser::SerializeStruct for FlatMapSerializeStruct<'a, M>
where
    M: SerializeMap + 'a,
{
    type Ok = ();
    type Error = M::Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.0.serialize_entry(key, value)
    }

    fn end(self) -> Result<(), Self::Error> {
        Ok(())
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(any(feature = "std", feature = "alloc"))]
pub struct FlatMapSerializeTupleVariantAsMapValue<'a, M: 'a> {
    map: &'a mut M,
    fields: Vec<Content>,
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl<'a, M> FlatMapSerializeTupleVariantAsMapValue<'a, M>
where
    M: SerializeMap + 'a,
{
    fn new(map: &'a mut M) -> Self {
        FlatMapSerializeTupleVariantAsMapValue {
            map,
            fields: Vec::new(),
        }
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl<'a, M> ser::SerializeTupleVariant for FlatMapSerializeTupleVariantAsMapValue<'a, M>
where
    M: SerializeMap + 'a,
{
    type Ok = ();
    type Error = M::Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        let value = tri!(value.serialize(ContentSerializer::<M::Error>::new()));
        self.fields.push(value);
        Ok(())
    }

    fn end(self) -> Result<(), Self::Error> {
        tri!(self.map.serialize_value(&Content::Seq(self.fields)));
        Ok(())
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(any(feature = "std", feature = "alloc"))]
pub struct FlatMapSerializeStructVariantAsMapValue<'a, M: 'a> {
    map: &'a mut M,
    name: &'static str,
    fields: Vec<(&'static str, Content)>,
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl<'a, M> FlatMapSerializeStructVariantAsMapValue<'a, M>
where
    M: SerializeMap + 'a,
{
    fn new(map: &'a mut M, name: &'static str) -> FlatMapSerializeStructVariantAsMapValue<'a, M> {
        FlatMapSerializeStructVariantAsMapValue {
            map,
            name,
            fields: Vec::new(),
        }
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl<'a, M> ser::SerializeStructVariant for FlatMapSerializeStructVariantAsMapValue<'a, M>
where
    M: SerializeMap + 'a,
{
    type Ok = ();
    type Error = M::Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        let value = tri!(value.serialize(ContentSerializer::<M::Error>::new()));
        self.fields.push((key, value));
        Ok(())
    }

    fn end(self) -> Result<(), Self::Error> {
        tri!(self
            .map
            .serialize_value(&Content::Struct(self.name, self.fields)));
        Ok(())
    }
}

pub struct AdjacentlyTaggedEnumVariant {
    pub enum_name: &'static str,
    pub variant_index: u32,
    pub variant_name: &'static str,
}

impl Serialize for AdjacentlyTaggedEnumVariant {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_unit_variant(self.enum_name, self.variant_index, self.variant_name)
    }
}

// Error when Serialize for a non_exhaustive remote enum encounters a variant
// that is not recognized.
pub struct CannotSerializeVariant<T>(pub T);

impl<T> Display for CannotSerializeVariant<T>
where
    T: Debug,
{
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "enum variant cannot be serialized: {:?}", self.0)
    }
}

#[cfg(test)]
mod rusty_tests {
	use crate::*;
	use std::default::Default;
	use std::clone::Clone;
	use std::cmp::PartialEq;
// #[test]
// #[should_panic]
// #[timeout(3000)]
// fn rusty_test_1() {
//     rusty_monitor::set_test_id(1);
//     let mut str_0: &str = "NYbr";
//     let mut str_0_ref_0: &str = &mut str_0;
//     let mut u16_0: u16 = 3273u16;
//     let mut str_1: &str = "al27QjY8StWb0w4KE5h";
//     let mut str_1_ref_0: &str = &mut str_1;
//     let mut u32_0: u32 = 482u32;
//     let mut str_2: &str = "wiH1RMrc";
//     let mut str_2_ref_0: &str = &mut str_2;
//     let mut u64_0: u64 = 6809u64;
//     let mut u64deserializer_0: crate::de::value::U64Deserializer<isize> = crate::de::value::U64Deserializer::new(u64_0);
//     let mut u64deserializer_0_ref_0: &crate::de::value::U64Deserializer<isize> = &mut u64deserializer_0;
//     let mut unsupported_0: __private::ser::Unsupported = crate::__private::ser::Unsupported::Boolean;
//     let mut u8_0: u8 = 99u8;
//     let mut u8deserializer_0: crate::de::value::U8Deserializer<isize> = crate::de::value::U8Deserializer::new(u8_0);
//     let mut u8deserializer_0_ref_0: &crate::de::value::U8Deserializer<isize> = &mut u8deserializer_0;
//     let mut i32_0: i32 = 5466i32;
//     let mut unsupported_1: __private::ser::Unsupported = crate::__private::ser::Unsupported::Integer;
//     let mut unsupported_1_ref_0: &__private::ser::Unsupported = &mut unsupported_1;
//     let mut content_0: __private::de::content::Content = crate::__private::de::content::Content::I32(i32_0);
//     let mut content_0_ref_0: &__private::de::content::Content = &mut content_0;
//     let mut content_1: __private::de::content::Content = crate::__private::de::content::Content::clone(content_0_ref_0);
//     let mut content_1_ref_0: &__private::de::content::Content = &mut content_1;
//     crate::__private::ser::FlatMapSerializer::bad_type(unsupported_0);
//     let mut content_2: __private::de::content::Content = crate::__private::de::content::Content::Unit;
//     let mut content_2_ref_0: &__private::de::content::Content = &mut content_2;
//     let mut unexpected_0: de::Unexpected = crate::de::Unexpected::Unit;
//     let mut unexpected_0_ref_0: &de::Unexpected = &mut unexpected_0;
//     let mut unexpected_1: de::Unexpected = crate::de::Unexpected::clone(unexpected_0_ref_0);
//     let mut content_3: __private::de::content::Content = crate::__private::de::content::Content::U16(u16_0);
//     let mut tagorcontentfield_0: __private::de::content::TagOrContentField = crate::__private::de::content::TagOrContentField::Tag;
//     panic!("From RustyUnit with love");
// }

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3() {
    rusty_monitor::set_test_id(3);
    let mut char_0: char = '6';
    let mut bool_0: bool = true;
    let mut booldeserializer_0: crate::de::value::BoolDeserializer<isize> = crate::de::value::BoolDeserializer::new(bool_0);
    let mut booldeserializer_0_ref_0: &crate::de::value::BoolDeserializer<isize> = &mut booldeserializer_0;
    let mut str_0: &str = "EJPKzqBmkFFZUFa";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "4KdgfSzDDyXDwBfqKBK";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_2: &str = "RfBQ9XK5qL";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut strdeserializer_0: crate::de::value::StrDeserializer<isize> = crate::de::value::StrDeserializer::new(str_2_ref_0);
    let mut strdeserializer_0_ref_0: &crate::de::value::StrDeserializer<isize> = &mut strdeserializer_0;
    let mut char_1: char = '-';
    let mut chardeserializer_0: crate::de::value::CharDeserializer<isize> = crate::de::value::CharDeserializer::new(char_1);
    let mut chardeserializer_0_ref_0: &crate::de::value::CharDeserializer<isize> = &mut chardeserializer_0;
    let mut u8_0: u8 = 49u8;
    let mut u8deserializer_0: crate::de::value::U8Deserializer<isize> = crate::de::value::U8Deserializer::new(u8_0);
    let mut u8deserializer_0_ref_0: &crate::de::value::U8Deserializer<isize> = &mut u8deserializer_0;
    let mut u32_0: u32 = 8333u32;
    let mut content_0: __private::de::content::Content = crate::__private::de::content::Content::U32(u32_0);
    let mut content_0_ref_0: &__private::de::content::Content = &mut content_0;
    let mut unsupported_0: __private::ser::Unsupported = crate::__private::ser::Unsupported::TupleStruct;
    let mut unsupported_0_ref_0: &__private::ser::Unsupported = &mut unsupported_0;
    let mut chardeserializer_1: crate::de::value::CharDeserializer<isize> = crate::de::value::CharDeserializer::clone(chardeserializer_0_ref_0);
    let mut chardeserializer_1_ref_0: &crate::de::value::CharDeserializer<isize> = &mut chardeserializer_1;
    let mut internallytaggedunitvisitor_0: crate::__private::de::content::InternallyTaggedUnitVisitor = crate::__private::de::content::InternallyTaggedUnitVisitor::new(str_1_ref_0, str_0_ref_0);
    let mut booldeserializer_1: crate::de::value::BoolDeserializer<isize> = crate::de::value::BoolDeserializer::clone(booldeserializer_0_ref_0);
    let mut booldeserializer_1_ref_0: &crate::de::value::BoolDeserializer<isize> = &mut booldeserializer_1;
    let mut content_1: __private::de::content::Content = crate::__private::de::content::Content::Char(char_0);
    let mut contentserializer_0: crate::__private::ser::content::ContentSerializer<isize> = crate::__private::ser::content::ContentSerializer::new();
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4() {
    rusty_monitor::set_test_id(4);
    let mut i64_0: i64 = 14406i64;
    let mut i64deserializer_0: crate::de::value::I64Deserializer<isize> = crate::de::value::I64Deserializer::new(i64_0);
    let mut i64deserializer_0_ref_0: &crate::de::value::I64Deserializer<isize> = &mut i64deserializer_0;
    let mut i64_1: i64 = 3039i64;
    let mut vec_0: std::vec::Vec<(&str, __private::ser::content::Content)> = std::vec::Vec::new();
    let mut str_0: &str = "nS5T906oSo";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut isize_0: isize = 3504isize;
    let mut str_1: &str = "qKsh";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut i16_0: i16 = -10666i16;
    let mut i16_1: i16 = 6922i16;
    let mut u8_0: u8 = 126u8;
    let mut i64_2: i64 = 2667i64;
    let mut i64deserializer_1: crate::de::value::I64Deserializer<isize> = crate::de::value::I64Deserializer::new(i64_2);
    let mut i64deserializer_1_ref_0: &crate::de::value::I64Deserializer<isize> = &mut i64deserializer_1;
    let mut i64deserializer_2: crate::de::value::I64Deserializer<isize> = crate::de::value::I64Deserializer::clone(i64deserializer_1_ref_0);
    let mut contentserializer_0: crate::__private::ser::content::ContentSerializer<isize> = crate::__private::ser::content::ContentSerializer::new();
    let mut contentserializer_1: crate::__private::ser::content::ContentSerializer<isize> = crate::__private::ser::content::ContentSerializer::new();
    let mut i64deserializer_2_ref_0: &crate::de::value::I64Deserializer<isize> = &mut i64deserializer_2;
    let mut i64deserializer_3: crate::de::value::I64Deserializer<isize> = crate::de::value::I64Deserializer::clone(i64deserializer_2_ref_0);
    let mut i64deserializer_3_ref_0: &crate::de::value::I64Deserializer<isize> = &mut i64deserializer_3;
    let mut i64deserializer_4: crate::de::value::I64Deserializer<isize> = crate::de::value::I64Deserializer::clone(i64deserializer_3_ref_0);
    let mut i64deserializer_4_ref_0: &crate::de::value::I64Deserializer<isize> = &mut i64deserializer_4;
    let mut i16deserializer_0: crate::de::value::I16Deserializer<isize> = crate::de::value::I16Deserializer::new(i16_1);
    let mut content_0: __private::ser::content::Content = crate::__private::ser::content::Content::I16(i16_0);
    let mut unexpected_0: de::Unexpected = crate::de::Unexpected::Str(str_1_ref_0);
    let mut serializestructvariantasmapvalue_0: crate::__private::ser::content::SerializeStructVariantAsMapValue<isize> = crate::__private::ser::content::SerializeStructVariantAsMapValue {map: isize_0, name: str_0_ref_0, fields: vec_0};
    let mut unexpected_1: de::Unexpected = crate::de::Unexpected::Signed(i64_1);
    let mut i64deserializer_5: crate::de::value::I64Deserializer<isize> = crate::de::value::I64Deserializer::clone(i64deserializer_0_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_11() {
    rusty_monitor::set_test_id(11);
    let mut u32_0: u32 = 4430u32;
    let mut u32deserializer_0: crate::de::value::U32Deserializer<isize> = crate::de::value::U32Deserializer::new(u32_0);
    let mut u32deserializer_0_ref_0: &crate::de::value::U32Deserializer<isize> = &mut u32deserializer_0;
    let mut str_0: &str = "plaU";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut u32_1: u32 = 1102u32;
    let mut str_1: &str = "eWAodPT3Nw29qeNCxkc";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut i8_0: i8 = 113i8;
    let mut i8deserializer_0: crate::de::value::I8Deserializer<isize> = crate::de::value::I8Deserializer::new(i8_0);
    let mut i8deserializer_0_ref_0: &crate::de::value::I8Deserializer<isize> = &mut i8deserializer_0;
    let mut isize_0: isize = -517isize;
    let mut unsupported_0: __private::ser::Unsupported = crate::__private::ser::Unsupported::TupleStruct;
    let mut unsupported_0_ref_0: &__private::ser::Unsupported = &mut unsupported_0;
    let mut mapaccessdeserializer_0: crate::de::value::MapAccessDeserializer<isize> = crate::de::value::MapAccessDeserializer::new(isize_0);
    let mut mapaccessdeserializer_0_ref_0: &crate::de::value::MapAccessDeserializer<isize> = &mut mapaccessdeserializer_0;
    let mut mapaccessdeserializer_1: crate::de::value::MapAccessDeserializer<isize> = crate::de::value::MapAccessDeserializer::clone(mapaccessdeserializer_0_ref_0);
    let mut mapaccessdeserializer_1_ref_0: &crate::de::value::MapAccessDeserializer<isize> = &mut mapaccessdeserializer_1;
    let mut mapaccessdeserializer_2: crate::de::value::MapAccessDeserializer<isize> = crate::de::value::MapAccessDeserializer::clone(mapaccessdeserializer_1_ref_0);
    let mut i8deserializer_1: crate::de::value::I8Deserializer<isize> = crate::de::value::I8Deserializer::clone(i8deserializer_0_ref_0);
    let mut i8deserializer_1_ref_0: &crate::de::value::I8Deserializer<isize> = &mut i8deserializer_1;
    let mut i8deserializer_2: crate::de::value::I8Deserializer<isize> = crate::de::value::I8Deserializer::clone(i8deserializer_1_ref_0);
    let mut i8deserializer_2_ref_0: &crate::de::value::I8Deserializer<isize> = &mut i8deserializer_2;
    let mut mapaccessdeserializer_2_ref_0: &crate::de::value::MapAccessDeserializer<isize> = &mut mapaccessdeserializer_2;
    let mut mapaccessdeserializer_3: crate::de::value::MapAccessDeserializer<isize> = crate::de::value::MapAccessDeserializer::clone(mapaccessdeserializer_2_ref_0);
    let mut mapaccessdeserializer_3_ref_0: &crate::de::value::MapAccessDeserializer<isize> = &mut mapaccessdeserializer_3;
    let mut mapaccessdeserializer_4: crate::de::value::MapAccessDeserializer<isize> = crate::de::value::MapAccessDeserializer::clone(mapaccessdeserializer_3_ref_0);
    let mut mapaccessdeserializer_4_ref_0: &crate::de::value::MapAccessDeserializer<isize> = &mut mapaccessdeserializer_4;
    let mut mapaccessdeserializer_5: crate::de::value::MapAccessDeserializer<isize> = crate::de::value::MapAccessDeserializer::clone(mapaccessdeserializer_4_ref_0);
    let mut mapaccessdeserializer_5_ref_0: &crate::de::value::MapAccessDeserializer<isize> = &mut mapaccessdeserializer_5;
    let mut mapaccessdeserializer_6: crate::de::value::MapAccessDeserializer<isize> = crate::de::value::MapAccessDeserializer::clone(mapaccessdeserializer_5_ref_0);
    panic!("From RustyUnit with love");
}

// #[test]
// #[should_panic]
// #[timeout(3000)]
// fn rusty_test_17() {
//     rusty_monitor::set_test_id(17);
//     let mut unsupported_0: __private::ser::Unsupported = crate::__private::ser::Unsupported::Optional;
//     let mut f64_0: f64 = 5207.790649f64;
//     let mut u8_0: u8 = 30u8;
//     let mut u8deserializer_0: crate::de::value::U8Deserializer<isize> = crate::de::value::U8Deserializer::new(u8_0);
//     let mut u8deserializer_0_ref_0: &crate::de::value::U8Deserializer<isize> = &mut u8deserializer_0;
//     let mut f64_1: f64 = -379.727618f64;
//     let mut isize_0: isize = 11073isize;
//     let mut option_0: std::option::Option<__private::ser::content::Content> = std::option::Option::None;
//     let mut i8_0: i8 = 21i8;
//     let mut i8deserializer_0: crate::de::value::I8Deserializer<isize> = crate::de::value::I8Deserializer::new(i8_0);
//     let mut i8deserializer_0_ref_0: &crate::de::value::I8Deserializer<isize> = &mut i8deserializer_0;
//     let mut i128_0: i128 = 544i128;
//     let mut i128deserializer_0: crate::de::value::I128Deserializer<isize> = crate::de::value::I128Deserializer::new(i128_0);
//     let mut i128deserializer_0_ref_0: &crate::de::value::I128Deserializer<isize> = &mut i128deserializer_0;
//     let mut unsupported_1: __private::ser::Unsupported = crate::__private::ser::Unsupported::Float;
//     let mut unsupported_1_ref_0: &__private::ser::Unsupported = &mut unsupported_1;
//     let mut i128deserializer_1: crate::de::value::I128Deserializer<isize> = crate::de::value::I128Deserializer::clone(i128deserializer_0_ref_0);
//     let mut i128deserializer_1_ref_0: &crate::de::value::I128Deserializer<isize> = &mut i128deserializer_1;
//     let mut content_0: __private::de::content::Content = crate::__private::de::content::Content::None;
//     let mut content_0_ref_0: &__private::de::content::Content = &mut content_0;
//     let mut option_1: std::option::Option<&str> = crate::__private::de::content::Content::as_str(content_0_ref_0);
//     let mut str_0: &str = std::option::Option::unwrap(option_1);
//     let mut content_1: __private::ser::content::Content = std::option::Option::unwrap(option_0);
//     let mut isizedeserializer_0: crate::de::value::IsizeDeserializer<isize> = crate::de::value::IsizeDeserializer::new(isize_0);
//     let mut isizedeserializer_0_ref_0: &crate::de::value::IsizeDeserializer<isize> = &mut isizedeserializer_0;
//     let mut f64deserializer_0: crate::de::value::F64Deserializer<isize> = crate::de::value::F64Deserializer::new(f64_1);
//     let mut content_2: __private::de::content::Content = crate::__private::de::content::Content::F64(f64_0);
//     let mut f64deserializer_0_ref_0: &crate::de::value::F64Deserializer<isize> = &mut f64deserializer_0;
//     let mut content_2_ref_0: &__private::de::content::Content = &mut content_2;
//     crate::__private::ser::FlatMapSerializer::bad_type(unsupported_0);
//     panic!("From RustyUnit with love");
// }

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_19() {
    rusty_monitor::set_test_id(19);
    let mut str_0: &str = "1XnGnFv9YE";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut u32_0: u32 = 8281u32;
    let mut str_1: &str = "xDVfBtqyQRb7IdLIU";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut f64_0: f64 = 4227.065699f64;
    let mut f64_1: f64 = -593.328321f64;
    let mut f64deserializer_0: crate::de::value::F64Deserializer<isize> = crate::de::value::F64Deserializer::new(f64_1);
    let mut f64deserializer_0_ref_0: &crate::de::value::F64Deserializer<isize> = &mut f64deserializer_0;
    let mut str_2: &str = "VMEcGC";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut f64_2: f64 = -17308.852054f64;
    let mut isize_0: isize = 4599isize;
    let mut isizedeserializer_0: crate::de::value::IsizeDeserializer<isize> = crate::de::value::IsizeDeserializer::new(isize_0);
    let mut isizedeserializer_0_ref_0: &crate::de::value::IsizeDeserializer<isize> = &mut isizedeserializer_0;
    let mut unsupported_0: __private::ser::Unsupported = crate::__private::ser::Unsupported::TupleStruct;
    let mut unsupported_0_ref_0: &__private::ser::Unsupported = &mut unsupported_0;
    let mut isizedeserializer_1: crate::de::value::IsizeDeserializer<isize> = crate::de::value::IsizeDeserializer::clone(isizedeserializer_0_ref_0);
    let mut content_0: __private::ser::content::Content = crate::__private::ser::content::Content::F64(f64_2);
    let mut tagcontentotherfield_0: __private::de::content::TagContentOtherField = crate::__private::de::content::TagContentOtherField::Other;
    let mut isizedeserializer_1_ref_0: &crate::de::value::IsizeDeserializer<isize> = &mut isizedeserializer_1;
    let mut isizedeserializer_2: crate::de::value::IsizeDeserializer<isize> = crate::de::value::IsizeDeserializer::clone(isizedeserializer_1_ref_0);
    let mut isizedeserializer_2_ref_0: &crate::de::value::IsizeDeserializer<isize> = &mut isizedeserializer_2;
    let mut isizedeserializer_3: crate::de::value::IsizeDeserializer<isize> = crate::de::value::IsizeDeserializer::clone(isizedeserializer_2_ref_0);
    let mut isizedeserializer_3_ref_0: &crate::de::value::IsizeDeserializer<isize> = &mut isizedeserializer_3;
    let mut f64deserializer_1: crate::de::value::F64Deserializer<isize> = crate::de::value::F64Deserializer::clone(f64deserializer_0_ref_0);
    let mut f64deserializer_2: crate::de::value::F64Deserializer<isize> = crate::de::value::F64Deserializer::new(f64_0);
    let mut f64deserializer_2_ref_0: &crate::de::value::F64Deserializer<isize> = &mut f64deserializer_2;
    let mut f64deserializer_1_ref_0: &crate::de::value::F64Deserializer<isize> = &mut f64deserializer_1;
    let mut f64deserializer_3: crate::de::value::F64Deserializer<isize> = crate::de::value::F64Deserializer::clone(f64deserializer_1_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_20() {
    rusty_monitor::set_test_id(20);
    let mut str_0: &str = "";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut i128_0: i128 = -12975i128;
    let mut u16_0: u16 = 7535u16;
    let mut mapaccessdeserializer_0: crate::de::value::MapAccessDeserializer<u16> = crate::de::value::MapAccessDeserializer::new(u16_0);
    let mut mapaccessdeserializer_0_ref_0: &crate::de::value::MapAccessDeserializer<u16> = &mut mapaccessdeserializer_0;
    let mut f32_0: f32 = -4410.888370f32;
    let mut i32_0: i32 = 1713i32;
    let mut isize_0: isize = 13189isize;
    let mut vec_0: std::vec::Vec<__private::ser::content::Content> = std::vec::Vec::new();
    let mut isize_1: isize = -8895isize;
    let mut isize_1_ref_0: &mut isize = &mut isize_1;
    let mut flatmapserializetuplevariantasmapvalue_0: crate::__private::ser::FlatMapSerializeTupleVariantAsMapValue<isize> = crate::__private::ser::FlatMapSerializeTupleVariantAsMapValue {map: isize_1_ref_0, fields: vec_0};
    let mut unexpected_0: de::Unexpected = crate::de::Unexpected::StructVariant;
    let mut unexpected_0_ref_0: &de::Unexpected = &mut unexpected_0;
    let mut unexpected_1: de::Unexpected = crate::de::Unexpected::clone(unexpected_0_ref_0);
    let mut unexpected_1_ref_0: &de::Unexpected = &mut unexpected_1;
    let mut mapaccessdeserializer_1: crate::de::value::MapAccessDeserializer<isize> = crate::de::value::MapAccessDeserializer::new(isize_0);
    let mut content_0: __private::de::content::Content = crate::__private::de::content::Content::I32(i32_0);
    let mut f32deserializer_0: crate::de::value::F32Deserializer<isize> = crate::de::value::F32Deserializer::new(f32_0);
    let mut content_0_ref_0: &__private::de::content::Content = &mut content_0;
    let mut mapaccessdeserializer_2: crate::de::value::MapAccessDeserializer<u16> = crate::de::value::MapAccessDeserializer::clone(mapaccessdeserializer_0_ref_0);
    let mut content_1: __private::ser::content::Content = crate::__private::ser::content::Content::Unit;
    let mut f32deserializer_0_ref_0: &crate::de::value::F32Deserializer<isize> = &mut f32deserializer_0;
    let mut i128deserializer_0: crate::de::value::I128Deserializer<isize> = crate::de::value::I128Deserializer::new(i128_0);
    let mut unexpected_2: de::Unexpected = crate::de::Unexpected::Other(str_0_ref_0);
    let mut unexpected_2_ref_0: &de::Unexpected = &mut unexpected_2;
    let mut mapaccessdeserializer_2_ref_0: &crate::de::value::MapAccessDeserializer<u16> = &mut mapaccessdeserializer_2;
    let mut mapaccessdeserializer_3: crate::de::value::MapAccessDeserializer<u16> = crate::de::value::MapAccessDeserializer::clone(mapaccessdeserializer_2_ref_0);
    let mut mapaccessdeserializer_3_ref_0: &crate::de::value::MapAccessDeserializer<u16> = &mut mapaccessdeserializer_3;
    let mut mapaccessdeserializer_4: crate::de::value::MapAccessDeserializer<u16> = crate::de::value::MapAccessDeserializer::clone(mapaccessdeserializer_3_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_27() {
    rusty_monitor::set_test_id(27);
    let mut i64_0: i64 = -5390i64;
    let mut i64_1: i64 = -16774i64;
    let mut i8_0: i8 = -54i8;
    let mut str_0: &str = "sg4KStuGsFBJYhdANm";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut borrowedstrdeserializer_0: crate::de::value::BorrowedStrDeserializer<isize> = crate::de::value::BorrowedStrDeserializer::new(str_0_ref_0);
    let mut borrowedstrdeserializer_0_ref_0: &crate::de::value::BorrowedStrDeserializer<isize> = &mut borrowedstrdeserializer_0;
    let mut vec_0: std::vec::Vec<__private::ser::content::Content> = std::vec::Vec::new();
    let mut str_1: &str = "P2va8dujqAndRI5JFL";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut isize_0: isize = -5959isize;
    let mut f32_0: f32 = -48.253285f32;
    let mut f32deserializer_0: crate::de::value::F32Deserializer<isize> = crate::de::value::F32Deserializer::new(f32_0);
    let mut f32deserializer_0_ref_0: &crate::de::value::F32Deserializer<isize> = &mut f32deserializer_0;
    let mut f32_1: f32 = 14292.857290f32;
    let mut u8_0: u8 = 60u8;
    let mut u8deserializer_0: crate::de::value::U8Deserializer<isize> = crate::de::value::U8Deserializer::new(u8_0);
    let mut u8deserializer_0_ref_0: &crate::de::value::U8Deserializer<isize> = &mut u8deserializer_0;
    let mut u8deserializer_1: crate::de::value::U8Deserializer<isize> = crate::de::value::U8Deserializer::clone(u8deserializer_0_ref_0);
    let mut u8deserializer_1_ref_0: &crate::de::value::U8Deserializer<isize> = &mut u8deserializer_1;
    let mut u8deserializer_2: crate::de::value::U8Deserializer<isize> = crate::de::value::U8Deserializer::clone(u8deserializer_1_ref_0);
    let mut u8deserializer_2_ref_0: &crate::de::value::U8Deserializer<isize> = &mut u8deserializer_2;
    let mut u8deserializer_3: crate::de::value::U8Deserializer<isize> = crate::de::value::U8Deserializer::clone(u8deserializer_2_ref_0);
    let mut u8deserializer_3_ref_0: &crate::de::value::U8Deserializer<isize> = &mut u8deserializer_3;
    let mut u8deserializer_4: crate::de::value::U8Deserializer<isize> = crate::de::value::U8Deserializer::clone(u8deserializer_3_ref_0);
    let mut u8deserializer_4_ref_0: &crate::de::value::U8Deserializer<isize> = &mut u8deserializer_4;
    let mut serializetuplevariantasmapvalue_0: crate::__private::ser::content::SerializeTupleVariantAsMapValue<isize> = crate::__private::ser::content::SerializeTupleVariantAsMapValue {map: isize_0, name: str_1_ref_0, fields: vec_0};
    let mut content_0: __private::ser::content::Content = crate::__private::ser::content::Content::I8(i8_0);
    let mut content_1: __private::ser::content::Content = crate::__private::ser::content::Content::I64(i64_1);
    let mut unexpected_0: de::Unexpected = crate::de::Unexpected::Signed(i64_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_29() {
    rusty_monitor::set_test_id(29);
    let mut str_0: &str = "PozvF3jZ30i";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut unsupported_0: __private::ser::Unsupported = crate::__private::ser::Unsupported::Char;
    let mut unsupported_0_ref_0: &__private::ser::Unsupported = &mut unsupported_0;
    let mut usize_0: usize = 9364usize;
    let mut i64_0: i64 = 1536i64;
    let mut u128_0: u128 = 1116u128;
    let mut u128deserializer_0: crate::de::value::U128Deserializer<isize> = crate::de::value::U128Deserializer::new(u128_0);
    let mut u128deserializer_0_ref_0: &crate::de::value::U128Deserializer<isize> = &mut u128deserializer_0;
    let mut u128_1: u128 = 2412u128;
    let mut vec_0: std::vec::Vec<__private::ser::content::Content> = std::vec::Vec::new();
    let mut vec_1: std::vec::Vec<__private::ser::content::Content> = std::vec::Vec::new();
    let mut str_1: &str = "HCWAM43liKz";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut usize_1: usize = 9647usize;
    let mut i64_1: i64 = -15156i64;
    let mut i64deserializer_0: crate::de::value::I64Deserializer<isize> = crate::de::value::I64Deserializer::new(i64_1);
    let mut i64deserializer_0_ref_0: &crate::de::value::I64Deserializer<isize> = &mut i64deserializer_0;
    let mut bool_0: bool = true;
    let mut str_2: &str = "KgikyDVngFT5xq6";
    let mut string_0: std::string::String = std::string::String::from(str_2);
    let mut content_0: __private::ser::content::Content = crate::__private::ser::content::Content::String(string_0);
    let mut i64deserializer_1: crate::de::value::I64Deserializer<isize> = crate::de::value::I64Deserializer::clone(i64deserializer_0_ref_0);
    let mut i64deserializer_1_ref_0: &crate::de::value::I64Deserializer<isize> = &mut i64deserializer_1;
    let mut usizedeserializer_0: crate::de::value::UsizeDeserializer<isize> = crate::de::value::UsizeDeserializer::new(usize_1);
    let mut usizedeserializer_0_ref_0: &crate::de::value::UsizeDeserializer<isize> = &mut usizedeserializer_0;
    let mut content_1: __private::ser::content::Content = crate::__private::ser::content::Content::TupleStruct(str_1_ref_0, vec_1);
    let mut content_2: __private::ser::content::Content = crate::__private::ser::content::Content::Tuple(vec_0);
    let mut u128deserializer_1: crate::de::value::U128Deserializer<isize> = crate::de::value::U128Deserializer::clone(u128deserializer_0_ref_0);
    let mut i64deserializer_2: crate::de::value::I64Deserializer<isize> = crate::de::value::I64Deserializer::new(i64_0);
    let mut unexpected_0: de::Unexpected = crate::de::Unexpected::Str(str_0_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_32() {
    rusty_monitor::set_test_id(32);
    let mut isize_0: isize = -92isize;
    let mut u64_0: u64 = 3060u64;
    let mut i8_0: i8 = 66i8;
    let mut isize_1: isize = 4719isize;
    let mut str_0: &str = "4XAdPxM";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "DAACuJJhQvRZeJLvIp";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_2: &str = "gZAChGjdlI4ktz";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_3: &str = "XCy1PTkQF10ODKRNDc";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut str_4: &str = "8Kjjdtzx";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut str_5: &str = "Kzx9KOrZ6yduyRfOPC";
    let mut string_0: std::string::String = std::string::String::from(str_5);
    let mut isize_2: isize = 2258isize;
    let mut isize_2_ref_0: &isize = &mut isize_2;
    let mut usize_0: usize = 2556usize;
    let mut isize_3: &isize = crate::__private::ser::constrain(isize_2_ref_0);
    let mut ignoredany_0: crate::de::ignored_any::IgnoredAny = crate::de::ignored_any::IgnoredAny::default();
    let mut ignoredany_0_ref_0: &crate::de::ignored_any::IgnoredAny = &mut ignoredany_0;
    let mut content_0: __private::de::content::Content = crate::__private::de::content::Content::String(string_0);
    let mut taggedserializer_0: crate::__private::ser::TaggedSerializer<isize> = crate::__private::ser::TaggedSerializer {type_ident: str_3_ref_0, variant_ident: str_2_ref_0, tag: str_1_ref_0, variant_name: str_0_ref_0, delegate: isize_1};
    let mut unexpected_0: de::Unexpected = crate::de::Unexpected::Seq;
    let mut content_1: __private::ser::content::Content = crate::__private::ser::content::Content::I8(i8_0);
    let mut unexpected_0_ref_0: &de::Unexpected = &mut unexpected_0;
    let mut u64deserializer_0: crate::de::value::U64Deserializer<isize> = crate::de::value::U64Deserializer::new(u64_0);
    let mut enumaccessdeserializer_0: crate::de::value::EnumAccessDeserializer<isize> = crate::de::value::EnumAccessDeserializer::new(isize_0);
    let mut content_0_ref_0: &__private::de::content::Content = &mut content_0;
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_33() {
    rusty_monitor::set_test_id(33);
    let mut str_0: &str = "Kx6ps1Jv7mP";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut content_0: __private::de::content::Content = crate::__private::de::content::Content::Unit;
    let mut content_0_ref_0: &__private::de::content::Content = &mut content_0;
    let mut str_1: &str = "k";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut u128_0: u128 = 8070u128;
    let mut i64_0: i64 = -8606i64;
    let mut isize_0: isize = -9695isize;
    let mut unitdeserializer_0: crate::de::value::UnitDeserializer<isize> = crate::de::value::UnitDeserializer::new();
    let mut unitdeserializer_0_ref_0: &crate::de::value::UnitDeserializer<isize> = &mut unitdeserializer_0;
    let mut i32_0: i32 = 5211i32;
    let mut i32deserializer_0: crate::de::value::I32Deserializer<isize> = crate::de::value::I32Deserializer::new(i32_0);
    let mut i32deserializer_0_ref_0: &crate::de::value::I32Deserializer<isize> = &mut i32deserializer_0;
    let mut i32_1: i32 = 15265i32;
    let mut isize_1: isize = -7378isize;
    let mut str_2: &str = "3lLeeyK3IcAPl6f5r";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut f64_0: f64 = -7024.045595f64;
    let mut content_1: __private::ser::content::Content = crate::__private::ser::content::Content::F64(f64_0);
    let mut borrowedstrdeserializer_0: crate::de::value::BorrowedStrDeserializer<isize> = crate::de::value::BorrowedStrDeserializer::new(str_2_ref_0);
    let mut enumaccessdeserializer_0: crate::de::value::EnumAccessDeserializer<isize> = crate::de::value::EnumAccessDeserializer::new(isize_1);
    let mut unsupported_0: __private::ser::Unsupported = crate::__private::ser::Unsupported::Integer;
    let mut borrowedstrdeserializer_0_ref_0: &crate::de::value::BorrowedStrDeserializer<isize> = &mut borrowedstrdeserializer_0;
    let mut borrowedstrdeserializer_1: crate::de::value::BorrowedStrDeserializer<isize> = crate::de::value::BorrowedStrDeserializer::clone(borrowedstrdeserializer_0_ref_0);
    let mut borrowedstrdeserializer_1_ref_0: &crate::de::value::BorrowedStrDeserializer<isize> = &mut borrowedstrdeserializer_1;
    let mut tuple_0: (isize, crate::de::value::private::UnitOnly<isize>) = crate::de::value::private::unit_only(isize_0);
    let mut unexpected_0: de::Unexpected = crate::de::Unexpected::Signed(i64_0);
    let mut u128deserializer_0: crate::de::value::U128Deserializer<isize> = crate::de::value::U128Deserializer::new(u128_0);
    let mut unexpected_0_ref_0: &de::Unexpected = &mut unexpected_0;
    let mut unsupported_0_ref_0: &__private::ser::Unsupported = &mut unsupported_0;
    let mut unexpected_1: de::Unexpected = crate::de::Unexpected::Other(str_0_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_37() {
    rusty_monitor::set_test_id(37);
    let mut isize_0: isize = 204isize;
    let mut str_0: &str = "50wqWWuFiPiMZzGejEQ";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "hf9YHyd";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_2: &str = "wViKFOdhcIQYnZFa2B";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_3: &str = "z8MQ";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut bool_0: bool = true;
    let mut booldeserializer_0: crate::de::value::BoolDeserializer<isize> = crate::de::value::BoolDeserializer::new(bool_0);
    let mut booldeserializer_0_ref_0: &crate::de::value::BoolDeserializer<isize> = &mut booldeserializer_0;
    let mut char_0: char = '`';
    let mut unitdeserializer_0: crate::de::value::UnitDeserializer<isize> = crate::de::value::UnitDeserializer::new();
    let mut unitdeserializer_0_ref_0: &crate::de::value::UnitDeserializer<isize> = &mut unitdeserializer_0;
    let mut str_4: &str = "EBy";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut str_5: &str = "bfWEWcst9DR8ZHpzdE";
    let mut str_5_ref_0: &str = &mut str_5;
    let mut unexpected_0: de::Unexpected = crate::de::Unexpected::Map;
    let mut unexpected_0_ref_0: &de::Unexpected = &mut unexpected_0;
    let mut bool_1: bool = true;
    let mut unexpected_1: de::Unexpected = crate::de::Unexpected::Bool(bool_1);
    let mut unexpected_1_ref_0: &de::Unexpected = &mut unexpected_1;
    let mut unsupported_0: __private::ser::Unsupported = crate::__private::ser::Unsupported::Char;
    let mut unsupported_0_ref_0: &__private::ser::Unsupported = &mut unsupported_0;
    let mut bool_2: bool = crate::de::Unexpected::eq(unexpected_1_ref_0, unexpected_0_ref_0);
    let mut unexpected_2: de::Unexpected = crate::de::Unexpected::TupleVariant;
    let mut unsupported_1: __private::ser::Unsupported = crate::__private::ser::Unsupported::ByteArray;
    let mut unexpected_2_ref_0: &de::Unexpected = &mut unexpected_2;
    let mut unsupported_1_ref_0: &__private::ser::Unsupported = &mut unsupported_1;
    let mut unexpected_3: de::Unexpected = crate::de::Unexpected::Unit;
    let mut unexpected_3_ref_0: &de::Unexpected = &mut unexpected_3;
    let mut unexpected_4: de::Unexpected = crate::de::Unexpected::clone(unexpected_3_ref_0);
    let mut unexpected_4_ref_0: &de::Unexpected = &mut unexpected_4;
    let mut tagcontentotherfieldvisitor_0: crate::__private::de::content::TagContentOtherFieldVisitor = crate::__private::de::content::TagContentOtherFieldVisitor {tag: str_5_ref_0, content: str_4_ref_0};
    let mut taggedserializer_0: crate::__private::ser::TaggedSerializer<isize> = crate::__private::ser::TaggedSerializer {type_ident: str_3_ref_0, variant_ident: str_2_ref_0, tag: str_1_ref_0, variant_name: str_0_ref_0, delegate: isize_0};
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_39() {
    rusty_monitor::set_test_id(39);
    let mut u64_0: u64 = 1196u64;
    let mut content_0: __private::de::content::Content = crate::__private::de::content::Content::U64(u64_0);
    let mut isize_0: isize = 450isize;
    let mut f64_0: f64 = 9861.573650f64;
    let mut unexpected_0: de::Unexpected = crate::de::Unexpected::Float(f64_0);
    let mut unexpected_0_ref_0: &de::Unexpected = &mut unexpected_0;
    let mut f64_1: f64 = 3589.409638f64;
    let mut i64_0: i64 = 11344i64;
    let mut unsupported_0: __private::ser::Unsupported = crate::__private::ser::Unsupported::TupleStruct;
    let mut unsupported_0_ref_0: &__private::ser::Unsupported = &mut unsupported_0;
    let mut char_0: char = 'P';
    let mut chardeserializer_0: crate::de::value::CharDeserializer<isize> = crate::de::value::CharDeserializer::new(char_0);
    let mut chardeserializer_0_ref_0: &crate::de::value::CharDeserializer<isize> = &mut chardeserializer_0;
    let mut char_1: char = 'W';
    let mut usize_0: usize = 5647usize;
    let mut str_0: &str = "BAZM";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut isize_1: isize = -6590isize;
    let mut i16_0: i16 = 2803i16;
    let mut f64_2: f64 = -3312.929652f64;
    let mut f64deserializer_0: crate::de::value::F64Deserializer<isize> = crate::de::value::F64Deserializer::new(f64_2);
    let mut content_1: __private::de::content::Content = crate::__private::de::content::Content::I16(i16_0);
    let mut content_1_ref_0: &__private::de::content::Content = &mut content_1;
    let mut serializestructvariantasmapvalue_0: crate::__private::ser::content::SerializeStructVariantAsMapValue<isize> = crate::__private::ser::content::SerializeStructVariantAsMapValue::new(isize_1, str_0_ref_0, usize_0);
    let mut f64deserializer_0_ref_0: &crate::de::value::F64Deserializer<isize> = &mut f64deserializer_0;
    let mut unexpected_1: de::Unexpected = crate::de::Unexpected::Signed(i64_0);
    let mut content_2: __private::ser::content::Content = crate::__private::ser::content::Content::F64(f64_1);
    let mut unexpected_1_ref_0: &de::Unexpected = &mut unexpected_1;
    let mut bool_0: bool = crate::de::Unexpected::eq(unexpected_1_ref_0, unexpected_0_ref_0);
    let mut tuple_0: (isize, crate::de::value::private::UnitOnly<isize>) = crate::de::value::private::unit_only(isize_0);
    let mut tagorcontent_0: __private::de::content::TagOrContent = crate::__private::de::content::TagOrContent::Content(content_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_40() {
    rusty_monitor::set_test_id(40);
    let mut unexpected_0: de::Unexpected = crate::de::Unexpected::UnitVariant;
    let mut unexpected_0_ref_0: &de::Unexpected = &mut unexpected_0;
    let mut f32_0: f32 = 11013.654381f32;
    let mut str_0: &str = "4YOJks2gaAXL";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut u32_0: u32 = 3879u32;
    let mut str_1: &str = "iWInalG";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_2: &str = "";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut f64_0: f64 = -1266.812798f64;
    let mut i16_0: i16 = 1421i16;
    let mut unsupported_0: __private::ser::Unsupported = crate::__private::ser::Unsupported::Float;
    let mut i16deserializer_0: crate::de::value::I16Deserializer<isize> = crate::de::value::I16Deserializer::new(i16_0);
    let mut content_0: __private::de::content::Content = crate::__private::de::content::Content::F64(f64_0);
    let mut content_0_ref_0: &__private::de::content::Content = &mut content_0;
    let mut content_1: __private::de::content::Content = crate::__private::de::content::Content::clone(content_0_ref_0);
    let mut i16deserializer_0_ref_0: &crate::de::value::I16Deserializer<isize> = &mut i16deserializer_0;
    let mut i16deserializer_1: crate::de::value::I16Deserializer<isize> = crate::de::value::I16Deserializer::clone(i16deserializer_0_ref_0);
    let mut content_2: __private::de::content::Content = crate::__private::de::content::Content::Str(str_2_ref_0);
    let mut content_1_ref_0: &__private::de::content::Content = &mut content_1;
    let mut unsupported_1: __private::ser::Unsupported = crate::__private::ser::Unsupported::Boolean;
    let mut unsupported_0_ref_0: &__private::ser::Unsupported = &mut unsupported_0;
    let mut content_3: __private::ser::content::Content = crate::__private::ser::content::Content::UnitVariant(str_1_ref_0, u32_0, str_0_ref_0);
    let mut unsupported_1_ref_0: &__private::ser::Unsupported = &mut unsupported_1;
    let mut content_4: __private::de::content::Content = crate::__private::de::content::Content::F32(f32_0);
    let mut content_2_ref_0: &__private::de::content::Content = &mut content_2;
    let mut content_5: __private::de::content::Content = crate::__private::de::content::Content::clone(content_2_ref_0);
    let mut unexpected_1: de::Unexpected = crate::de::Unexpected::clone(unexpected_0_ref_0);
    let mut unexpected_1_ref_0: &de::Unexpected = &mut unexpected_1;
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_47() {
    rusty_monitor::set_test_id(47);
    let mut isize_0: isize = 2569isize;
    let mut i32_0: i32 = 2225i32;
    let mut str_0: &str = "Byg9H66767KQ";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "ryyfytnLp9uiD";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut borrowedstrdeserializer_0: crate::de::value::BorrowedStrDeserializer<isize> = crate::de::value::BorrowedStrDeserializer::new(str_1_ref_0);
    let mut borrowedstrdeserializer_0_ref_0: &crate::de::value::BorrowedStrDeserializer<isize> = &mut borrowedstrdeserializer_0;
    let mut vec_0: std::vec::Vec<u8> = std::vec::Vec::new();
    let mut isize_1: isize = 4819isize;
    let mut isizedeserializer_0: crate::de::value::IsizeDeserializer<isize> = crate::de::value::IsizeDeserializer::new(isize_1);
    let mut isizedeserializer_0_ref_0: &crate::de::value::IsizeDeserializer<isize> = &mut isizedeserializer_0;
    let mut isize_2: isize = -6204isize;
    let mut i8_0: i8 = 114i8;
    let mut i8deserializer_0: crate::de::value::I8Deserializer<isize> = crate::de::value::I8Deserializer::new(i8_0);
    let mut i8deserializer_0_ref_0: &crate::de::value::I8Deserializer<isize> = &mut i8deserializer_0;
    let mut contentserializer_0: crate::__private::ser::content::ContentSerializer<isize> = crate::__private::ser::content::ContentSerializer::new();
    let mut content_0: __private::ser::content::Content = crate::__private::ser::content::Content::Bytes(vec_0);
    let mut unsupported_0: __private::ser::Unsupported = crate::__private::ser::Unsupported::String;
    let mut unsupported_0_ref_0: &__private::ser::Unsupported = &mut unsupported_0;
    let mut unexpected_0: de::Unexpected = crate::de::Unexpected::Other(str_0_ref_0);
    let mut unexpected_0_ref_0: &de::Unexpected = &mut unexpected_0;
    let mut unexpected_1: de::Unexpected = crate::de::Unexpected::clone(unexpected_0_ref_0);
    let mut i32deserializer_0: crate::de::value::I32Deserializer<isize> = crate::de::value::I32Deserializer::new(i32_0);
    let mut unexpected_1_ref_0: &de::Unexpected = &mut unexpected_1;
    let mut unexpected_2: de::Unexpected = crate::de::Unexpected::clone(unexpected_1_ref_0);
    let mut i32deserializer_0_ref_0: &crate::de::value::I32Deserializer<isize> = &mut i32deserializer_0;
    let mut i32deserializer_1: crate::de::value::I32Deserializer<isize> = crate::de::value::I32Deserializer::clone(i32deserializer_0_ref_0);
    let mut unexpected_2_ref_0: &de::Unexpected = &mut unexpected_2;
    let mut i32deserializer_1_ref_0: &crate::de::value::I32Deserializer<isize> = &mut i32deserializer_1;
    let mut isizedeserializer_1: crate::de::value::IsizeDeserializer<isize> = crate::de::value::IsizeDeserializer::new(isize_0);
    panic!("From RustyUnit with love");
}
}