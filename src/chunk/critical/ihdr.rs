use crate::chunk::chunk::Chunk;
use std::convert::TryInto;
use crate::utility::rules::invalid_chunk_type;
use std::fmt;

#[derive(Copy, Clone)]
pub struct Ihdr {
    pub width: u32,
    pub height: u32,
    pub bit_depth: u8,
    pub color_type: u8,
    pub compression_method: u8,
    pub filter_method: u8,
    pub interlace_method: u8,
}

impl Ihdr {
    pub fn data(chunk: &Chunk) -> Option<Ihdr> {
        let t = chunk.ascii_type();
        match &*t {
            "IHDR" => {
                Some(Ihdr {
                    width: u32::from_be_bytes(chunk.raw_data[0..=3].try_into().unwrap()),
                    height: u32::from_be_bytes(chunk.raw_data[4..=7].try_into().unwrap()),
                    bit_depth: chunk.raw_data[8],
                    color_type: chunk.raw_data[9],
                    compression_method: chunk.raw_data[10],
                    filter_method: chunk.raw_data[11],
                    interlace_method: chunk.raw_data[12]
                })
            },
            _ => { None }
        }
    }
}

impl fmt::Debug for Ihdr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let w = format!("{}", self.width);
        let h = format!("{}", self.height);

        write!(f, "width/height: {}/{}, bit depth: {}, color type: {}, compression: {}, filter: {}, interlace: {}",
               w, h, self.bit_depth, self.color_type, self.compression_method, self.filter_method, self.interlace_method)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_data() {
        let data = [
            0x00, 0x00, 0x00, 0xc8,
            0x00, 0x00, 0x00, 0x64,
            0x01, 0x02, 0x03, 0x04,
            0x05
        ];

        let chunk = Chunk {
            len: 0,
            raw_type: 0x49484452,
            raw_data: data.to_vec(),
            crc: 0
        };

        let ihdr = Ihdr::data(&chunk).unwrap();
        assert_eq!(ihdr.width, 200);
        assert_eq!(ihdr.height, 100);
        assert_eq!(ihdr.bit_depth, 1);
        assert_eq!(ihdr.color_type, 2);
        assert_eq!(ihdr.compression_method, 3);
        assert_eq!(ihdr.filter_method, 4);
        assert_eq!(ihdr.interlace_method, 5);
    }
}