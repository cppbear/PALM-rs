// Answer 0

#[derive(Debug)]
struct MyStruct {
    config: ConfigType,
}

impl MyStruct {
    fn config(&self) -> &ConfigType {
        &self.config
    }
}

#[derive(Debug)]
struct ConfigType {
    value: String,
}

#[test]
fn test_config() {
    let config_instance = ConfigType {
        value: String::from("Test Configuration"),
    };
    
    let my_struct_instance = MyStruct {
        config: config_instance,
    };
    
    assert_eq!(my_struct_instance.config().value, "Test Configuration");
}

#[test]
fn test_config_empty() {
    let config_instance = ConfigType {
        value: String::from(""),
    };
    
    let my_struct_instance = MyStruct {
        config: config_instance,
    };
    
    assert_eq!(my_struct_instance.config().value, "");
}

