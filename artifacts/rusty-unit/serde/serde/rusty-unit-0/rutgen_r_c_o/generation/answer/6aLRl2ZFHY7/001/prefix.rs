// Answer 0

#[test]
fn test_next_value_seed_bool() {
    let mut access = FlatStructAccess {
        iter: std::slice::IterMut::empty(),
        pending_content: Some(Content::Bool(true)),
        fields: &["field"],
        _marker: PhantomData,
    };
    let seed = BoolSeed;
    let _ = access.next_value_seed(seed);
}

#[test]
fn test_next_value_seed_u8() {
    let mut access = FlatStructAccess {
        iter: std::slice::IterMut::empty(),
        pending_content: Some(Content::U8(0)),
        fields: &["field"],
        _marker: PhantomData,
    };
    let seed = U8Seed;
    let _ = access.next_value_seed(seed);
}

#[test]
fn test_next_value_seed_u16() {
    let mut access = FlatStructAccess {
        iter: std::slice::IterMut::empty(),
        pending_content: Some(Content::U16(0)),
        fields: &["field"],
        _marker: PhantomData,
    };
    let seed = U16Seed;
    let _ = access.next_value_seed(seed);
}

#[test]
fn test_next_value_seed_u32() {
    let mut access = FlatStructAccess {
        iter: std::slice::IterMut::empty(),
        pending_content: Some(Content::U32(0)),
        fields: &["field"],
        _marker: PhantomData,
    };
    let seed = U32Seed;
    let _ = access.next_value_seed(seed);
}

#[test]
fn test_next_value_seed_u64() {
    let mut access = FlatStructAccess {
        iter: std::slice::IterMut::empty(),
        pending_content: Some(Content::U64(0)),
        fields: &["field"],
        _marker: PhantomData,
    };
    let seed = U64Seed;
    let _ = access.next_value_seed(seed);
}

#[test]
fn test_next_value_seed_i8() {
    let mut access = FlatStructAccess {
        iter: std::slice::IterMut::empty(),
        pending_content: Some(Content::I8(0)),
        fields: &["field"],
        _marker: PhantomData,
    };
    let seed = I8Seed;
    let _ = access.next_value_seed(seed);
}

#[test]
fn test_next_value_seed_i16() {
    let mut access = FlatStructAccess {
        iter: std::slice::IterMut::empty(),
        pending_content: Some(Content::I16(0)),
        fields: &["field"],
        _marker: PhantomData,
    };
    let seed = I16Seed;
    let _ = access.next_value_seed(seed);
}

#[test]
fn test_next_value_seed_i32() {
    let mut access = FlatStructAccess {
        iter: std::slice::IterMut::empty(),
        pending_content: Some(Content::I32(0)),
        fields: &["field"],
        _marker: PhantomData,
    };
    let seed = I32Seed;
    let _ = access.next_value_seed(seed);
}

#[test]
fn test_next_value_seed_i64() {
    let mut access = FlatStructAccess {
        iter: std::slice::IterMut::empty(),
        pending_content: Some(Content::I64(0)),
        fields: &["field"],
        _marker: PhantomData,
    };
    let seed = I64Seed;
    let _ = access.next_value_seed(seed);
}

#[test]
fn test_next_value_seed_f32() {
    let mut access = FlatStructAccess {
        iter: std::slice::IterMut::empty(),
        pending_content: Some(Content::F32(0.0)),
        fields: &["field"],
        _marker: PhantomData,
    };
    let seed = F32Seed;
    let _ = access.next_value_seed(seed);
}

#[test]
fn test_next_value_seed_f64() {
    let mut access = FlatStructAccess {
        iter: std::slice::IterMut::empty(),
        pending_content: Some(Content::F64(0.0)),
        fields: &["field"],
        _marker: PhantomData,
    };
    let seed = F64Seed;
    let _ = access.next_value_seed(seed);
}

#[test]
fn test_next_value_seed_char() {
    let mut access = FlatStructAccess {
        iter: std::slice::IterMut::empty(),
        pending_content: Some(Content::Char('a')),
        fields: &["field"],
        _marker: PhantomData,
    };
    let seed = CharSeed;
    let _ = access.next_value_seed(seed);
}

#[test]
fn test_next_value_seed_string() {
    let mut access = FlatStructAccess {
        iter: std::slice::IterMut::empty(),
        pending_content: Some(Content::String(String::from("test"))),
        fields: &["field"],
        _marker: PhantomData,
    };
    let seed = StringSeed;
    let _ = access.next_value_seed(seed);
}

#[test]
fn test_next_value_seed_bytes() {
    let mut access = FlatStructAccess {
        iter: std::slice::IterMut::empty(),
        pending_content: Some(Content::Bytes(&[1, 2, 3])),
        fields: &["field"],
        _marker: PhantomData,
    };
    let seed = BytesSeed;
    let _ = access.next_value_seed(seed);
}

#[test]
fn test_next_value_seed_none() {
    let mut access = FlatStructAccess {
        iter: std::slice::IterMut::empty(),
        pending_content: Some(Content::None),
        fields: &["field"],
        _marker: PhantomData,
    };
    let seed = NoneSeed;
    let _ = access.next_value_seed(seed);
}

#[test]
fn test_next_value_seed_some() {
    let mut access = FlatStructAccess {
        iter: std::slice::IterMut::empty(),
        pending_content: Some(Content::Some(Box::new(Content::Unit))),
        fields: &["field"],
        _marker: PhantomData,
    };
    let seed = SomeSeed;
    let _ = access.next_value_seed(seed);
}

#[test]
fn test_next_value_seed_unit() {
    let mut access = FlatStructAccess {
        iter: std::slice::IterMut::empty(),
        pending_content: Some(Content::Unit),
        fields: &["field"],
        _marker: PhantomData,
    };
    let seed = UnitSeed;
    let _ = access.next_value_seed(seed);
}

#[test]
fn test_next_value_seed_newtype() {
    let mut access = FlatStructAccess {
        iter: std::slice::IterMut::empty(),
        pending_content: Some(Content::Newtype(Box::new(Content::U8(255)))),
        fields: &["field"],
        _marker: PhantomData,
    };
    let seed = NewtypeSeed;
    let _ = access.next_value_seed(seed);
}

#[test]
fn test_next_value_seed_seq() {
    let mut access = FlatStructAccess {
        iter: std::slice::IterMut::empty(),
        pending_content: Some(Content::Seq(vec![Content::U32(1), Content::U32(2)])),
        fields: &["field"],
        _marker: PhantomData,
    };
    let seed = SeqSeed;
    let _ = access.next_value_seed(seed);
}

#[test]
fn test_next_value_seed_map() {
    let mut access = FlatStructAccess {
        iter: std::slice::IterMut::empty(),
        pending_content: Some(Content::Map(vec![(Content::String(String::from("key")), Content::U8(1))])),
        fields: &["field"],
        _marker: PhantomData,
    };
    let seed = MapSeed;
    let _ = access.next_value_seed(seed);
}

