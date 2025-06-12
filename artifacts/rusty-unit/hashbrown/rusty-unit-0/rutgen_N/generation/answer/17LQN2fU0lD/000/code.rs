// Answer 0

#[cfg(test)]
mod tests {
    use super::*;
    use std::fmt;

    struct TestStruct {
        data: Vec<i32>,
    }

    impl TestStruct {
        fn new(data: Vec<i32>) -> Self {
            TestStruct { data }
        }
    }

    impl fmt::Debug for TestStruct {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_list().entries(self.data.iter()).finish()
        }
    }

    #[test]
    fn test_fmt_debug_list() {
        let test_data = TestStruct::new(vec![1, 2, 3]);
        let output = format!("{:?}", test_data);
        assert_eq!(output, "[1, 2, 3]");
    }

    #[test]
    fn test_fmt_empty() {
        let test_data = TestStruct::new(vec![]);
        let output = format!("{:?}", test_data);
        assert_eq!(output, "[]");
    }

    #[test]
    fn test_fmt_single_element() {
        let test_data = TestStruct::new(vec![42]);
        let output = format!("{:?}", test_data);
        assert_eq!(output, "[42]");
    }
}

