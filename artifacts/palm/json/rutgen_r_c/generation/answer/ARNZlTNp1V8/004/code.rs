// Answer 0

#[test]
fn test_deserialize_any_unit() {
    struct TestVisitor;
    
    impl<'de> de::Visitor<'de> for TestVisitor {
        type Value = ();
        
        fn visit_unit(self) -> Result<Self::Value> {
            Ok(())
        }
        
        // Other visit methods can be left unimplemented as they won't be called.
        #[inline(always)] fn visit_bool(self, _: bool) -> Result<Self::Value> { unreachable!() }
        #[inline(always)] fn visit_str(self, _: &str) -> Result<Self::Value> { unreachable!() }
        #[inline(always)] fn visit_borrowed_str(self, _: &'de str) -> Result<Self::Value> { unreachable!() }
        #[inline(always)] fn visit_seq<V>(self, _: V) -> Result<Self::Value> where V: de::SeqVisitor<'de> { unreachable!() }
        #[inline(always)] fn visit_map<V>(self, _: V) -> Result<Self::Value> where V: de::MapVisitor<'de> { unreachable!() }
        #[inline(always)] fn visit_f64(self, _: f64) -> Result<Self::Value> { unreachable!() }
        #[inline(always)] fn visit_u64(self, _: u64) -> Result<Self::Value> { unreachable!() }
        #[inline(always)] fn visit_i64(self, _: i64) -> Result<Self::Value> { unreachable!() }
    }

    let mut deserializer = Deserializer {
        read: StrRead::new("null".as_bytes()),
        scratch: Vec::new(),
        remaining_depth: 128, // assume a high enough depth
    };

    assert_eq!(deserializer.deserialize_any(TestVisitor), Ok(()));
}

#[test]
fn test_deserialize_any_bool_true() {
    struct TestVisitor;
    
    impl<'de> de::Visitor<'de> for TestVisitor {
        type Value = bool;

        fn visit_bool(self, value: bool) -> Result<Self::Value> {
            Ok(value)
        }
        
        // Other visit methods are unimplemented.
        #[inline(always)] fn visit_unit(self) -> Result<Self::Value> { unreachable!() }
        #[inline(always)] fn visit_str(self, _: &str) -> Result<Self::Value> { unreachable!() }
        #[inline(always)] fn visit_borrowed_str(self, _: &'de str) -> Result<Self::Value> { unreachable!() }
        #[inline(always)] fn visit_seq<V>(self, _: V) -> Result<Self::Value> where V: de::SeqVisitor<'de> { unreachable!() }
        #[inline(always)] fn visit_map<V>(self, _: V) -> Result<Self::Value> where V: de::MapVisitor<'de> { unreachable!() }
        #[inline(always)] fn visit_f64(self, _: f64) -> Result<Self::Value> { unreachable!() }
        #[inline(always)] fn visit_u64(self, _: u64) -> Result<Self::Value> { unreachable!() }
        #[inline(always)] fn visit_i64(self, _: i64) -> Result<Self::Value> { unreachable!() }
    }

    let mut deserializer = Deserializer {
        read: StrRead::new("true".as_bytes()),
        scratch: Vec::new(),
        remaining_depth: 128,
    };
    
    assert_eq!(deserializer.deserialize_any(TestVisitor), Ok(true));
}

#[test]
fn test_deserialize_any_bool_false() {
    struct TestVisitor;

    impl<'de> de::Visitor<'de> for TestVisitor {
        type Value = bool;

        fn visit_bool(self, value: bool) -> Result<Self::Value> {
            Ok(value)
        }

        // Other visit methods are unimplemented.
        #[inline(always)] fn visit_unit(self) -> Result<Self::Value> { unreachable!() }
        #[inline(always)] fn visit_str(self, _: &str) -> Result<Self::Value> { unreachable!() }
        #[inline(always)] fn visit_borrowed_str(self, _: &'de str) -> Result<Self::Value> { unreachable!() }
        #[inline(always)] fn visit_seq<V>(self, _: V) -> Result<Self::Value> where V: de::SeqVisitor<'de> { unreachable!() }
        #[inline(always)] fn visit_map<V>(self, _: V) -> Result<Self::Value> where V: de::MapVisitor<'de> { unreachable!() }
        #[inline(always)] fn visit_f64(self, _: f64) -> Result<Self::Value> { unreachable!() }
        #[inline(always)] fn visit_u64(self, _: u64) -> Result<Self::Value> { unreachable!() }
        #[inline(always)] fn visit_i64(self, _: i64) -> Result<Self::Value> { unreachable!() }
    }

    let mut deserializer = Deserializer {
        read: StrRead::new("false".as_bytes()),
        scratch: Vec::new(),
        remaining_depth: 128,
    };

    assert_eq!(deserializer.deserialize_any(TestVisitor), Ok(false));
}

#[test]
fn test_deserialize_any_number_negative() {
    struct TestVisitor;

    impl<'de> de::Visitor<'de> for TestVisitor {
        type Value = i64;

        fn visit_i64(self, value: i64) -> Result<Self::Value> {
            Ok(value)
        }
        
        // Other visit methods are unimplemented.
        #[inline(always)] fn visit_unit(self) -> Result<Self::Value> { unreachable!() }
        #[inline(always)] fn visit_bool(self, _: bool) -> Result<Self::Value> { unreachable!() }
        #[inline(always)] fn visit_str(self, _: &str) -> Result<Self::Value> { unreachable!() }
        #[inline(always)] fn visit_borrowed_str(self, _: &'de str) -> Result<Self::Value> { unreachable!() }
        #[inline(always)] fn visit_seq<V>(self, _: V) -> Result<Self::Value> where V: de::SeqVisitor<'de> { unreachable!() }
        #[inline(always)] fn visit_map<V>(self, _: V) -> Result<Self::Value> where V: de::MapVisitor<'de> { unreachable!() }
        #[inline(always)] fn visit_f64(self, _: f64) -> Result<Self::Value> { unreachable!() }
        #[inline(always)] fn visit_u64(self, _: u64) -> Result<Self::Value> { unreachable!() }
    }

    let mut deserializer = Deserializer {
        read: StrRead::new("-123".as_bytes()),
        scratch: Vec::new(),
        remaining_depth: 128,
    };

    assert_eq!(deserializer.deserialize_any(TestVisitor), Ok(-123));
}

#[test]
fn test_deserialize_any_string() {
    struct TestVisitor;

    impl<'de> de::Visitor<'de> for TestVisitor {
        type Value = String;

        fn visit_str(self, value: &str) -> Result<Self::Value> {
            Ok(value.to_owned())
        }

        fn visit_borrowed_str(self, value: &'de str) -> Result<Self::Value> {
            Ok(value.to_owned())
        }
        
        // Other visit methods are unimplemented.
        #[inline(always)] fn visit_unit(self) -> Result<Self::Value> { unreachable!() }
        #[inline(always)] fn visit_bool(self, _: bool) -> Result<Self::Value> { unreachable!() }
        #[inline(always)] fn visit_seq<V>(self, _: V) -> Result<Self::Value> where V: de::SeqVisitor<'de> { unreachable!() }
        #[inline(always)] fn visit_map<V>(self, _: V) -> Result<Self::Value> where V: de::MapVisitor<'de> { unreachable!() }
        #[inline(always)] fn visit_f64(self, _: f64) -> Result<Self::Value> { unreachable!() }
        #[inline(always)] fn visit_u64(self, _: u64) -> Result<Self::Value> { unreachable!() }
        #[inline(always)] fn visit_i64(self, _: i64) -> Result<Self::Value> { unreachable!() }
    }

    let mut deserializer = Deserializer {
        read: StrRead::new("\"test\"".as_bytes()),
        scratch: Vec::new(),
        remaining_depth: 128,
    };

    assert_eq!(deserializer.deserialize_any(TestVisitor), Ok("test".to_owned()));
}

#[test]
fn test_deserialize_any_seq() {
    struct TestVisitor;

    impl<'de> de::Visitor<'de> for TestVisitor {
        type Value = ();

        fn visit_seq<V>(self, _: V) -> Result<Self::Value>
        where
            V: de::SeqVisitor<'de>,
        {
            Ok(())
        }
        
        // Other visit methods are unimplemented.
        #[inline(always)] fn visit_unit(self) -> Result<Self::Value> { unreachable!() }
        #[inline(always)] fn visit_bool(self, _: bool) -> Result<Self::Value> { unreachable!() }
        #[inline(always)] fn visit_str(self, _: &str) -> Result<Self::Value> { unreachable!() }
        #[inline(always)] fn visit_borrowed_str(self, _: &'de str) -> Result<Self::Value> { unreachable!() }
        #[inline(always)] fn visit_map<V>(self, _: V) -> Result<Self::Value> where V: de::MapVisitor<'de> { unreachable!() }
        #[inline(always)] fn visit_f64(self, _: f64) -> Result<Self::Value> { unreachable!() }
        #[inline(always)] fn visit_u64(self, _: u64) -> Result<Self::Value> { unreachable!() }
        #[inline(always)] fn visit_i64(self, _: i64) -> Result<Self::Value> { unreachable!() }
    }

    let mut deserializer = Deserializer {
        read: StrRead::new("[1, 2, 3]".as_bytes()),
        scratch: Vec::new(),
        remaining_depth: 128,
    };

    assert_eq!(deserializer.deserialize_any(TestVisitor), Ok(()));
}

#[test]
fn test_deserialize_any_map() {
    struct TestVisitor;

    impl<'de> de::Visitor<'de> for TestVisitor {
        type Value = ();

        fn visit_map<V>(self, _: V) -> Result<Self::Value>
        where
            V: de::MapVisitor<'de>,
        {
            Ok(())
        }
        
        // Other visit methods are unimplemented.
        #[inline(always)] fn visit_unit(self) -> Result<Self::Value> { unreachable!() }
        #[inline(always)] fn visit_bool(self, _: bool) -> Result<Self::Value> { unreachable!() }
        #[inline(always)] fn visit_str(self, _: &str) -> Result<Self::Value> { unreachable!() }
        #[inline(always)] fn visit_borrowed_str(self, _: &'de str) -> Result<Self::Value> { unreachable!() }
        #[inline(always)] fn visit_seq<V>(self, _: V) -> Result<Self::Value> where V: de::SeqVisitor<'de> { unreachable!() }
        #[inline(always)] fn visit_f64(self, _: f64) -> Result<Self::Value> { unreachable!() }
        #[inline(always)] fn visit_u64(self, _: u64) -> Result<Self::Value> { unreachable!() }
        #[inline(always)] fn visit_i64(self, _: i64) -> Result<Self::Value> { unreachable!() }
    }

    let mut deserializer = Deserializer {
        read: StrRead::new("{\"key\": 1}".as_bytes()),
        scratch: Vec::new(),
        remaining_depth: 128,
    };

    assert_eq!(deserializer.deserialize_any(TestVisitor), Ok(()));
}

#[test]
#[should_panic]
fn test_deserialize_any_invalid_input() {
    struct TestVisitor;

    impl<'de> de::Visitor<'de> for TestVisitor {
        type Value = ();

        fn visit_unit(self) -> Result<Self::Value> {
            unreachable!()
        }
        
        // Other visit methods can be implemented here if necessary.
    }

    let mut deserializer = Deserializer {
        read: StrRead::new("invalid_input".as_bytes()),
        scratch: Vec::new(),
        remaining_depth: 128,
    };

    deserializer.deserialize_any(TestVisitor).unwrap();
}

