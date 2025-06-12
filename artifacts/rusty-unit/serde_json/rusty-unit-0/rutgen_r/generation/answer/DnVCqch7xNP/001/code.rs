// Answer 0

#[test]
fn test_serialize_unit_struct() {
    struct UnitStruct;

    impl serde::ser::Serialize for UnitStruct {
        fn serialize<S>(&self, _: S) -> Result<(), S::Error>
        where
            S: serde::ser::Serializer,
        {
            self.serialize_unit_struct("UnitStructName")
        }
    }

    let result = UnitStruct.serialize_unit_struct("UnitStructName");
    assert_eq!(result, Err(key_must_be_a_string()));
}

#[test]
#[should_panic]
fn test_serialize_unit_struct_panic() {
    struct PanicStruct;

    impl serde::ser::Serialize for PanicStruct {
        fn serialize<S>(&self, _: S) -> Result<(), S::Error>
        where
            S: serde::ser::Serializer,
        {
            panic!("This should trigger a panic for test!");
        }
    }

    let _ = PanicStruct.serialize_unit_struct("PanicStructName");
}

