use std::{fmt, io};
use std::str::from_utf8;
use std::fs::File;
use std::io::Read;
use std::convert::TryInto;
use crate::chunk::critical::ihdr::Ihdr;

pub struct Chunk {
    pub len: u32,
    pub raw_type: u32,
    pub raw_data: Vec<u8>,
    pub crc: u32
}

impl Chunk {
    pub fn ascii_type(&self) -> String {
        let t = self.raw_type.to_be_bytes();
        from_utf8(&t).unwrap().to_string()
    }
}

impl fmt::Debug for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let l = format!("{:08X}", self.len);
        let d_count = self.raw_data.iter().count().to_string();
        let c = format!("{:08X}", self.crc);
        let t = self.ascii_type();

        let d = match &*t {
            "IHDR" => { Some(Ihdr::data(self)) },
            _ => { None },
        };

        let k = d.map(|d| format!("{:?}", d)).unwrap_or("".to_string());

        write!(f, "\n[{}]\nlen: {}, data: {}, crc: {}\n", self.ascii_type(), l, k, c)
    }
}

pub fn gen_chunk(url: &str) -> io::Result<Vec<Chunk>> {
    let mut file = File::open(url)?;
    let mut bytes: Vec<u8> = Vec::new();
    let mut chunks: Vec<Chunk> = Vec::new();

    file.read_to_end(&mut bytes)?;
    read_chunk_bytes(&mut chunks, bytes, 8);

    Ok(chunks)
}

pub fn read_chunk_bytes(chunks: &mut Vec<Chunk>, bytes: Vec<u8>, i: usize) {
    let raw_length = &bytes[i ..= i + 3];
    let raw_type = &bytes[i + 4 ..= i + 7];

    let l = u32::from_be_bytes(raw_length.try_into().unwrap());
    let t = u32::from_be_bytes(raw_type.try_into().unwrap());

    let last_i = i + 11 + l as usize;

    let (data, crc) = if l == 0 {
        (Vec::new(), &bytes[i + 8 ..= last_i])

    } else {
        let d = bytes[i + 8 ..= i + 7 + l as usize].iter().cloned().collect::<Vec<u8>>();
        let c =  &bytes[i + 8 + l as usize ..= last_i];
        (d, c)
    };

    let c = u32::from_be_bytes(crc.try_into().unwrap());

    let chunk = Chunk {
        len: l,
        raw_type: t,
        raw_data: data,
        crc: c
    };

    chunks.push(chunk);

    if l != 0 && last_i != bytes.len() - 1 {
        read_chunk_bytes(chunks, bytes, last_i + 1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_read_chunk_bytes() {
        let mut chunks: Vec<Chunk> = Vec::new();
        let data = [
            0x00, 0x00, 0x00, 0x0d,
            0x49, 0x48, 0x44, 0x52,
            0x00, 0x00, 0x00, 0xc8,
            0x00, 0x00, 0x00, 0xc8,
            0x08, 0x06, 0x00, 0x00,
            0x00, 0xad, 0x58, 0xae,
            0x9eu8
        ];
        read_chunk_bytes(&mut chunks, data.to_vec(), 0);
        assert_eq!(chunks[0].len, 13);
        assert_eq!(chunks[0].raw_type.to_be_bytes(), [0x49u8, 0x48, 0x44, 0x52]);
    }
}