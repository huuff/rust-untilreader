use std::io::Result;

pub trait UntilReader {
    fn read_until_string(&mut self, ending: &str, buf: Vec<u8>) -> Result<usize>;
}


