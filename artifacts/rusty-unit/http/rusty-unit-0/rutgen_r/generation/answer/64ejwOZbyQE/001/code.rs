// Answer 0

#[test]
fn test_extension_with_string() {
    struct Builder {
        extensions: std::collections::HashMap<&'static str, Box<dyn std::any::Any + Send + Sync>>,
    }

    impl Builder {
        pub fn new() -> Self {
            Builder {
                extensions: std::collections::HashMap::new(),
            }
        }

        pub fn extension<T>(mut self, extension: T) -> Result<Self, Box<dyn std::any::Any + Send + Sync>>
        where
            T: Clone + std::any::Any + Send + Sync + 'static,
        {
            self.extensions.insert(std::any::type_name::<T>(), Box::new(extension));
            Ok(self)
        }

        pub fn extensions(&self) -> &std::collections::HashMap<&'static str, Box<dyn std::any::Any + Send + Sync>> {
            &self.extensions
        }
    }

    let builder = Builder::new();
    let response = builder.extension("My Extension").unwrap();

    assert_eq!(response.extensions().get(std::any::type_name::<&'static str>()), Some(&Box::new("My Extension") as Box<dyn std::any::Any + Send + Sync>));
}

#[test]
fn test_extension_with_integer() {
    struct Builder {
        extensions: std::collections::HashMap<&'static str, Box<dyn std::any::Any + Send + Sync>>,
    }

    impl Builder {
        pub fn new() -> Self {
            Builder {
                extensions: std::collections::HashMap::new(),
            }
        }

        pub fn extension<T>(mut self, extension: T) -> Result<Self, Box<dyn std::any::Any + Send + Sync>>
        where
            T: Clone + std::any::Any + Send + Sync + 'static,
        {
            self.extensions.insert(std::any::type_name::<T>(), Box::new(extension));
            Ok(self)
        }

        pub fn extensions(&self) -> &std::collections::HashMap<&'static str, Box<dyn std::any::Any + Send + Sync>> {
            &self.extensions
        }
    }

    let builder = Builder::new();
    let response = builder.extension(42).unwrap();

    assert_eq!(response.extensions().get(std::any::type_name::<i32>()), Some(&Box::new(42) as Box<dyn std::any::Any + Send + Sync>));
}

#[should_panic]
#[test]
fn test_extension_with_invalid_type() {
    struct Builder {
        extensions: std::collections::HashMap<&'static str, Box<dyn std::any::Any + Send + Sync>>,
    }

    impl Builder {
        pub fn new() -> Self {
            Builder {
                extensions: std::collections::HashMap::new(),
            }
        }

        pub fn extension<T>(mut self, _extension: T) -> Result<Self, Box<dyn std::any::Any + Send + Sync>>
        where
            T: Clone + std::any::Any + Send + Sync + 'static,
        {
            panic!("This should panic for demonstration purposes.");
        }
        
        pub fn extensions(&self) -> &std::collections::HashMap<&'static str, Box<dyn std::any::Any + Send + Sync>> {
            &self.extensions
        }
    }

    let builder = Builder::new();
    let _ = builder.extension("Trigger Panic").unwrap();
}

