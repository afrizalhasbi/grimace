use std::fs::File;
use std::io::{BufReader, Bytes, Read};
use std::str;

pub struct ByteIterator {
    bytes: Bytes<BufReader<File>>,
}

impl ByteIterator {
    pub fn from_file(file: &str) -> ByteIterator {
        let file = File::open(file).unwrap();
        let bytes = BufReader::new(file).bytes();
        ByteIterator { bytes: bytes }
    }
    pub fn next(&mut self) -> u8 {
        self.bytes.next().unwrap().unwrap()
    }
    pub fn nextn(&mut self, size: usize) -> Vec<u8> {
        let mut i = 0;
        let mut vec = Vec::with_capacity(size);
        while i < size {
            vec.push(self.bytes.next().unwrap().unwrap())
        }
        vec
    }
}

pub fn read_file(file: &str) -> BufReader<File> {
    let file = File::open(file).unwrap();
    BufReader::new(file)
}

pub fn scale_u8u16(value: u8) -> u16 {
    let value = value as u16;
    value * (65535 / 255)
}

pub fn byte_to_ascii(bytes: &[u8]) -> &str {
    let s = str::from_utf8(bytes);
    match s {
        Ok(ascii) => return ascii,
        Err(e) => panic!("Error casting byte to ascii: {}", e),
    };
}

pub fn iter_bits(byte: u8) -> Vec<u8> {
    (0..8).map(|i| (byte >> (7 - i)) & 1).collect()
}
