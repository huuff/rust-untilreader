use std::io::{ Result, BufReader, Read, BufRead, };
use std::str;

pub trait UntilReader {
    fn read_until_string(&mut self, ending: &str, buf: Vec<u8>) -> Result<usize>;
}

impl <T: Read> UntilReader for BufReader<T> {
    fn read_until_string(&mut self, ending: &str, buf: Vec<u8>) -> Result<usize> {
        let fill_buf = self.fill_buf()?;

        if fill_buf.is_empty() {
            return Ok(0);
        }

        let mut consumed = 0;
        loop {
            // The buffer is finished, so we can just break
            if consumed == fill_buf.len() {
                break;
            }

            // The remainings of the buffer are longer than or equal to the string we're looking for
            if consumed <= (fill_buf.len() - ending.len()) {
                // Then check wether the next few bytes are the string we're looking for
                let next_str = &fill_buf[consumed..(consumed+ending.len())];
                // TODO: No unwrapping!
                // TODO: Might be better if I just convert the ending to a byte array?
                let next_str = str::from_utf8(next_str).unwrap();
                consumed += ending.len();
                
                if next_str == ending {
                    break;
                }
            } else {
                // Just read the rest of the buffer
                todo!();
            }
        }

        // TOO: Uuuh was a string in my original one
        buf.push_str(str::from_utf8(&fill_buf[..consumed]).unwrap());
        self.consume(consumed);
        return Ok(consumed);

    }
}
