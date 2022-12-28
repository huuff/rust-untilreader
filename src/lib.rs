use std::io::{ Result, BufReader, Read, BufRead, };

pub trait UntilReader {
    fn read_until_bytes(&mut self, ending: &[u8], buf: &mut Vec<u8>) -> Result<usize>;
}

impl <T: Read> UntilReader for BufReader<T> {
    fn read_until_bytes(&mut self, ending: &[u8], buf: &mut Vec<u8>) -> Result<usize> {
        let fill_buf = self.fill_buf()?;

        if fill_buf.is_empty() {
            return Ok(0);
        }

        let mut consumed = 0;
        // TODO: Can I use a while?
        loop {
            // The buffer is finished, so we can just break
            if consumed == fill_buf.len() {
                break;
            }

            // The remainings of the buffer are longer than or equal to the string we're looking for
            if consumed <= (fill_buf.len() - ending.len()) {
                // Then check wether the next few bytes are the string we're looking for
                let next_str = &fill_buf[consumed..(consumed+ending.len())];
                consumed += ending.len();
                
                if next_str == ending {
                    break;
                }
            } else {
                // Just read the rest of the buffer
                todo!("I don't even think this is necessary");
            }
        }

        buf.extend_from_slice(&fill_buf[..consumed]);
        self.consume(consumed);
        return Ok(consumed);

    }
}

// TODO: Port al tests from `crlf` (the current one is the first one from crlf)
// TODO: Remove unwraps from tests (return results)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_crlf_line() {
        // ARRANGE
        let mut buf = Vec::new();
        let mut buf_reader = BufReader::new(
            "This is a text\r\nwith two lines".as_bytes()
        );

        // ACT
        let result = buf_reader.read_until_bytes("\r\n".as_bytes(), &mut buf);

        // ASSERT
        assert!(result.is_ok());
        assert_eq!(String::from_utf8(buf).unwrap(), "This is a text\r\n");
        assert_eq!(result.unwrap(), 16);
    }

    #[test]
    fn read_two_crlf_lines() {
        let mut buf = Vec::new();
        let mut buf_reader =  BufReader::new(
            "This is a text\r\nwith three lines\r\nseparated by crlf".as_bytes()
        );

        let first_result = buf_reader.read_until_bytes("\r\n".as_bytes(), &mut buf);
        assert!(first_result.is_ok());
        assert_eq!(String::from_utf8(buf.clone()).unwrap(), "This is a text\r\n");
        assert_eq!(first_result.unwrap(), 16);

        buf.clear(); 

        let second_result = buf_reader.read_until_bytes("\r\n".as_bytes(), &mut buf);
        assert!(second_result.is_ok());
        assert_eq!(String::from_utf8(buf).unwrap(), "with three lines\r\n");
        assert_eq!(second_result.unwrap(), 18);
    }
}
