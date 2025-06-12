// Answer 0

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::ser::Serializer;
    use serde::Serialize;

    struct UnitStruct;

    impl Serialize for UnitStruct {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            serializer.serialize_unit()
        }
    }

    #[test]
    fn test_serialize_unit_struct() {
        let unit_struct = UnitStruct;
        let result = serde_json::to_string(&unit_struct);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "null");
    }
}

