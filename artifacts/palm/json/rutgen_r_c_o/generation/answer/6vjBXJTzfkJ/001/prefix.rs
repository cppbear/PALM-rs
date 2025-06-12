// Answer 0

#[test]
fn test_io_read_new_with_non_empty_file() {
    use std::fs::File;
    use std::io::{BufReader, Read};

    let file = File::create("test_file_1.json").unwrap();
    let mut writer = std::io::BufWriter::new(file);
    writer.write_all(b"{\"key\":\"value\"}").unwrap();
    writer.flush().unwrap();

    let file = File::open("test_file_1.json").unwrap();
    let io_read = IoRead::new(BufReader::new(file));
}

#[test]
fn test_io_read_new_with_empty_file() {
    use std::fs::File;
    use std::io::{BufReader, Read};

    let file = File::create("test_file_2.json").unwrap();
    let mut writer = std::io::BufWriter::new(file);
    writer.flush().unwrap();

    let file = File::open("test_file_2.json").unwrap();
    let io_read = IoRead::new(BufReader::new(file));
}

#[test]
#[should_panic]
fn test_io_read_new_with_large_file() {
    use std::fs::File;
    use std::io::{BufReader, Read};

    let file = File::create("test_file_3.json").unwrap();
    let mut writer = std::io::BufWriter::new(file);
    writer.write_all(&vec![0; 65537]).unwrap(); // write more than 65536 bytes
    writer.flush().unwrap();

    let file = File::open("test_file_3.json").unwrap();
    let _io_read = IoRead::new(BufReader::new(file));
}

#[test]
fn test_io_read_new_with_standard_stream() {
    use std::io::{self, BufReader};

    let input = io::Cursor::new(b"{\"key\":\"value\"}");
    let io_read = IoRead::new(BufReader::new(input));
}

#[test]
fn test_io_read_new_with_small_bytes() {
    use std::io::{self, BufReader};

    let input = io::Cursor::new(b"abc");
    let io_read = IoRead::new(BufReader::new(input));
}

