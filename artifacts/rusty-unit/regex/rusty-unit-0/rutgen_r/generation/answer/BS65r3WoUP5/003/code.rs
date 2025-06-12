// Answer 0

#[test]
fn test_cmd_compile_success() {
    struct Args {
        flag_bytes: bool,
        flag_dfa: bool,
        flag_dfa_reverse: bool,
    }

    impl Args {
        fn parse_many(&self) -> Result<Vec<String>, &'static str> {
            Ok(vec![String::from("test"), String::from("abc")])
        }

        fn compiler(&self) -> Compiler {
            Compiler {
                flag_bytes: self.flag_bytes,
                flag_dfa: self.flag_dfa,
                flag_dfa_reverse: self.flag_dfa_reverse,
            }
        }
    }

    struct Compiler {
        flag_bytes: bool,
        flag_dfa: bool,
        flag_dfa_reverse: bool,
    }

    impl Compiler {
        fn bytes(self, _flag: bool) -> Self {
            self
        }

        fn only_utf8(self, _flag: bool) -> Self {
            self
        }

        fn dfa(self, _flag: bool) -> Self {
            self
        }

        fn reverse(self, _flag: bool) -> Self {
            self
        }

        fn compile(&self, _exprs: &[String]) -> Result<String, &'static str> {
            Ok(String::from("compiled_program"))
        }
    }

    fn cmd_compile(args: &Args) -> Result<(), &'static str> {
        let exprs = args.parse_many()?;
        let compiler = args.compiler()
            .bytes(args.flag_bytes)
            .only_utf8(!args.flag_bytes)
            .dfa(args.flag_dfa)
            .reverse(args.flag_dfa_reverse);
        let prog = compiler.compile(&exprs)?;
        print!("{:?}", prog);
        Ok(())
    }

    let args = Args { flag_bytes: false, flag_dfa: true, flag_dfa_reverse: false };
    let result = cmd_compile(&args);
    assert_eq!(result, Ok(()));
}

