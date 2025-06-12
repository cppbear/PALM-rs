// Answer 0

fn cmd_captures(args: &Args) -> Result<()> {
    let expr = args.parse_one()?;
    let prog = args.compiler().only_utf8(false).compile(&[expr])?;
    for (i, name) in prog.captures.iter().enumerate() {
        match *name {
            None => println!("{}", i),
            Some(ref name) => println!("{}:{}", i, name),
        }
    }
    Ok(())
}

#[derive(Debug)]
struct TestArgs {
    expr: String,
    captures: Vec<Option<String>>,
}

impl TestArgs {
    fn parse_one(&self) -> Result<String, &'static str> {
        Ok(self.expr.clone())
    }
    
    fn compiler(&self) -> TestCompiler {
        TestCompiler {
            captures: self.captures.clone()
        }
    }
}

#[derive(Debug)]
struct TestCompiler {
    captures: Vec<Option<String>>,
}

impl TestCompiler {
    fn only_utf8(self, _: bool) -> Self {
        self
    }
    
    fn compile(self, _: &[String]) -> Result<TestProg, &'static str> {
        Ok(TestProg {
            captures: self.captures
        })
    }
}

#[derive(Debug)]
struct TestProg {
    captures: Vec<Option<String>>,
}

type Result<T> = std::result::Result<T, &'static str>;

#[test]
fn test_cmd_captures_with_some_names() {
    let args = TestArgs {
        expr: String::from("a"),
        captures: vec![Some(String::from("first")), Some(String::from("second"))],
    };
    
    let result = cmd_captures(&args);
    assert!(result.is_ok());
}

#[test]
fn test_cmd_captures_with_none_names() {
    let args = TestArgs {
        expr: String::from("b"),
        captures: vec![None, None, Some(String::from("third"))],
    };
    
    let result = cmd_captures(&args);
    assert!(result.is_ok());
}

#[test]
fn test_cmd_captures_with_mixed_names() {
    let args = TestArgs {
        expr: String::from("c"),
        captures: vec![Some(String::from("fourth")), None, Some(String::from("fifth"))],
    };
    
    let result = cmd_captures(&args);
    assert!(result.is_ok());
}

