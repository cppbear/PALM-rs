// Answer 0

#[derive(Debug)]
struct Writer;
#[derive(Debug)]
struct Formatter<'a> {
    writer: &'a mut Writer,
}

#[derive(Debug)]
struct Ser {
    formatter: Formatter<'static>,
}

#[derive(Debug)]
enum State {
    Empty,
    NonEmpty,
}

#[derive(Debug)]
enum Compound {
    Map { ser: Ser, state: State },
}

#[derive(Debug)]
enum Error {
    Io(std::io::Error),
}

type Result<T> = std::result::Result<T, Error>;

impl<'a> Formatter<'a> {
    fn end_array(&mut self, _writer: &mut Writer) -> Result<()> {
        Ok(())
    }
}

impl Ser {
    fn new(writer: &'static mut Writer) -> Self {
        Ser {
            formatter: Formatter { writer },
        }
    }
}

impl Compound {
    fn end(self) -> Result<()> {
        match self {
            Compound::Map { ser, state } => match state {
                State::Empty => Ok(()),
                _ => ser.formatter.end_array(&mut ser.writer).map_err(Error::Io),
            },
        }
    }
}

#[test]
fn test_end_non_empty_state() {
    let mut writer = Writer;
    let ser = Ser::new(&mut writer);
    let compound = Compound::Map { ser, state: State::NonEmpty };
    
    let result = compound.end();
    assert!(result.is_ok());
}

#[test]
#[should_panic]
fn test_end_empty_state() {
    let mut writer = Writer;
    let ser = Ser::new(&mut writer);
    let compound = Compound::Map { ser, state: State::Empty };
    
    let _result = compound.end(); // This should panic
}

