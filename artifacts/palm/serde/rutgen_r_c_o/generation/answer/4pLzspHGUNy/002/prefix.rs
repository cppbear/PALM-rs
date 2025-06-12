// Answer 0

#[test]
fn test_deserialize_any_with_visitor_ok_and_end_err() {
    struct TestVisitor {
        called: bool,
    }

    impl<'de> de::Visitor<'de> for TestVisitor {
        type Value = u32; // for example
        
        fn visit_seq<V>(self, _: &mut V) -> Result<Self::Value, V::Error>
        where
            V: de::SeqAccess<'de>,
        {
            self.called = true; 
            Ok(42) // return some valid value
        }
    }

    let values = vec![1, 2, 3];    
    let deserializer = SeqDeserializer {
        iter: values.into_iter().fuse(),
        count: 0,
        marker: PhantomData,
    };
    
    let visitor = TestVisitor { called: false };
    
    let result = deserializer.deserialize_any(visitor);
}

#[test]
fn test_deserialize_any_with_visitor_ok_and_end_count_non_zero() {
    struct TestVisitor {
        called: bool,
    }

    impl<'de> de::Visitor<'de> for TestVisitor {
        type Value = u32; 
        
        fn visit_seq<V>(self, _: &mut V) -> Result<Self::Value, V::Error>
        where
            V: de::SeqAccess<'de>,
        {
            self.called = true; 
            Ok(42) // return some valid value
        }
    }

    let values = vec![1, 2, 3];    
    let mut deserializer = SeqDeserializer {
        iter: values.into_iter().fuse(),
        count: 5, // induce a remaining count > 0
        marker: PhantomData,
    };
    
    let visitor = TestVisitor { called: false };
    
    let result = deserializer.deserialize_any(visitor);
}

#[test]
#[should_panic]
fn test_deserialize_any_with_visitor_ok_and_end_nonzero_count() {
    struct TestVisitor {
        called: bool,
    }

    impl<'de> de::Visitor<'de> for TestVisitor {
        type Value = u32; 
        
        fn visit_seq<V>(self, _: &mut V) -> Result<Self::Value, V::Error>
        where
            V: de::SeqAccess<'de>,
        {
            self.called = true; 
            Ok(42) // return some valid value
        }
    }

    let values = vec![1, 2, 3];    
    let mut deserializer = SeqDeserializer {
        iter: values.into_iter().fuse(),
        count: 3, // does not match previous count
        marker: PhantomData,
    };
    
    let visitor = TestVisitor { called: false };
    
    let _result = deserializer.deserialize_any(visitor);
}

