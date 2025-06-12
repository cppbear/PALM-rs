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