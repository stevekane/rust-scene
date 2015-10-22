use std::io::Read;

pub struct Chars<R:Read> { 
    reader: R, 
    buf: [u8;1]
}

impl <R:Read> Chars<R> {
    pub fn new(r: R) -> Chars<R> {
        Chars { reader: r, buf: [0;1] }
    }
}

impl <R:Read> Iterator for Chars<R> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        match self.reader.read(&mut self.buf) {
            Ok(0) | Err(_) => None,
            Ok(c)          => Some(self.buf[0] as char),
        }
    }
}
