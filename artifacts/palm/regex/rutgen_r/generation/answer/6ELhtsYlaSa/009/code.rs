// Answer 0

fn run(args: &Args) -> Result<()> {
    if args.cmd_ast {
        cmd_ast(args)
    } else if args.cmd_hir {
        cmd_hir(args)
    } else if args.cmd_prefixes {
        cmd_literals(args)
    } else if args.cmd_suffixes {
        cmd_literals(args)
    } else if args.cmd_anchors {
        cmd_anchors(args)
    } else if args.cmd_captures {
        cmd_captures(args)
    } else if args.cmd_compile {
        cmd_compile(args)
    } else if args.cmd_utf8_ranges {
        cmd_utf8_ranges(args)
    } else {
        unreachable!()
    }
}

#[derive(Default)]
struct Args {
    cmd_ast: bool,
    cmd_hir: bool,
    cmd_prefixes: bool,
    cmd_suffixes: bool,
    cmd_anchors: bool,
    cmd_captures: bool,
    cmd_compile: bool,
    cmd_utf8_ranges: bool,
}

#[test]
#[should_panic]
fn test_run_unreachable() {
    let args = Args::default();
    run(&args).unwrap();
}

