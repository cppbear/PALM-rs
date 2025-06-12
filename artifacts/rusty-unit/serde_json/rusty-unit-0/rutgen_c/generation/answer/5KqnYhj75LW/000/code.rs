// Answer 0

#[test]
fn test_unit_variant() {
    struct TestVariantAccess<'a> {
        _marker: std::marker::PhantomData<&'a ()>,
    }

    impl<'de> de::VariantAccess<'de> for TestVariantAccess<'de> {
        type Error = Error;

        fn unit_variant(self) -> Result<(), Error> {
            Ok(())
        }

        fn newtype_variant_seed<T>(self, _seed: T) -> Result<T::Value, Error>
        where
            T: de::DeserializeSeed<'de>,
        {
            unimplemented!()
        }

        fn tuple_variant<V>(self, _len: usize, _visitor: V) -> Result<V::Value, Error>
        where
            V: de::Visitor<'de>,
        {
            unimplemented!()
        }

        fn struct_variant<V>(
            self,
            _fields: &'static [&'static str],
            _visitor: V,
        ) -> Result<V::Value, Error>
        where
            V: de::Visitor<'de>,
        {
            unimplemented!()
        }
    }

    let variant_access = TestVariantAccess {
        _marker: std::marker::PhantomData,
    };

    let result = variant_access.unit_variant();
    assert!(result.is_ok());
}

#[test]
#[should_panic]
fn test_unit_variant_panic() {
    struct TestVariantAccessPanic<'a> {
        _marker: std::marker::PhantomData<&'a ()>,
    }

    impl<'de> de::VariantAccess<'de> for TestVariantAccessPanic<'de> {
        type Error = Error;

        fn unit_variant(self) -> Result<(), Error> {
            Err(Error)
        }

        fn newtype_variant_seed<T>(self, _seed: T) -> Result<T::Value, Error>
        where
            T: de::DeserializeSeed<'de>,
        {
            unimplemented!()
        }

        fn tuple_variant<V>(self, _len: usize, _visitor: V) -> Result<V::Value, Error>
        where
            V: de::Visitor<'de>,
        {
            unimplemented!()
        }

        fn struct_variant<V>(
            self,
            _fields: &'static [&'static str],
            _visitor: V,
        ) -> Result<V::Value, Error>
        where
            V: de::Visitor<'de>,
        {
            unimplemented!()
        }
    }

    let variant_access = TestVariantAccessPanic {
        _marker: std::marker::PhantomData,
    };

    let _result = variant_access.unit_variant().expect("Expected an error");
}

