// Answer 0

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Serialize;
    use serde_json::ser;

    #[test]
    fn test_serialize_element() {
        struct TestSerializer {
            data: Vec<String>,
        }

        impl ser::SerializeSeq for TestSerializer {
            type Ok = ();
            type Error = serde_json::Error;

            fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
            where
                T: ?Sized + Serialize,
            {
                let serialized = serde_json::to_string(value)?;
                self.data.push(serialized);
                Ok(())
            }

            fn end(self) -> Result<Self::Ok, Self::Error> {
                Ok(())
            }
        }

        let mut serializer = TestSerializer { data: vec![] };
        let value = "test";

        let result = serializer.serialize_element(&value);
        assert!(result.is_ok());
        assert_eq!(serializer.data.len(), 1);
        assert_eq!(serializer.data[0], "\"test\"");
    }

    #[test]
    fn test_serialize_element_with_struct() {
        #[derive(Serialize)]
        struct TestStruct {
            field: i32,
        }

        struct TestSerializer {
            data: Vec<String>,
        }

        impl ser::SerializeSeq for TestSerializer {
            type Ok = ();
            type Error = serde_json::Error;

            fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
            where
                T: ?Sized + Serialize,
            {
                let serialized = serde_json::to_string(value)?;
                self.data.push(serialized);
                Ok(())
            }

            fn end(self) -> Result<Self::Ok, Self::Error> {
                Ok(())
            }
        }

        let mut serializer = TestSerializer { data: vec![] };
        let value = TestStruct { field: 42 };

        let result = serializer.serialize_element(&value);
        assert!(result.is_ok());
        assert_eq!(serializer.data.len(), 1);
        assert_eq!(serializer.data[0], "{\"field\":42}");
    }
}

