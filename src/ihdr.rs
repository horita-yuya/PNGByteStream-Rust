use std::fmt;
use std::intrinsics::transmute;
use std::str::from_utf8;

#[derive(Debug)]
pub struct IHDR {
    pub width: u32,
    pub height: u32,
    pub bit_depth: u8,
    pub color_type: u8,
    pub compression_method: u8,
    pub filter_method: u8,
    pub interlace_method: u8,
}

pub struct Chunk {
    pub len: u32,
    pub raw_type: u32,
    pub raw_data: Vec<u8>,
    pub crc: u32
}

impl fmt::Debug for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let l = format!("{:08X}", self.len);
        let t = format!("{:08X}", self.raw_type);
        let d_count = self.raw_data.iter().count().to_string();
        let c = format!("{:08X}", self.crc);

//        let ds = unsafe {
//            transmute::<u32, [u8; 4]>(self.raw_type.to_be())
//        };
        let ds = self.raw_type.to_be_bytes();

        let str = from_utf8(&ds).unwrap();

        write!(f, "\n[{}]\nlen: {}, data: {} byte, crc: {}\n", str, l, d_count, c)
    }
}