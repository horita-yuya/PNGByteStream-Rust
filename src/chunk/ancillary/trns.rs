use crate::chunk::chunk::Chunk;
use crate::chunk::ancillary::trns::Trns::{Gray, RGB, Palettes};
use std::convert::TryInto;

pub enum Trns {
    Gray(u16),
    RGB(u16, u16, u16),
    Palettes(Vec<u8>),
}

impl Trns {
    pub fn data(chunk: Chunk, color_type: u8) -> Option<Trns> {
        let t = chunk.ascii_type();

        if t == "tRNS".to_string() {
            None
        } else {
            match color_type {
                0 => {
                    let gray = u16::from_be_bytes(chunk.raw_data[0..=1].try_into().unwrap());
                    Some(Gray(gray))
                },
                2 => {
                    let r = u16::from_be_bytes(chunk.raw_data[0..=1].try_into().unwrap());
                    let g = u16::from_be_bytes(chunk.raw_data[2..=3].try_into().unwrap());
                    let b = u16::from_be_bytes(chunk.raw_data[4..=5].try_into().unwrap());
                    Some(RGB(r, g, b))
                }
                3 => {
                    Some(Palettes(chunk.raw_data))
                }
                _ => {
                    None
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_data_gray() {
        let t = *b"tRNS";

        let chunk = Chunk {
            len: 0,
            raw_type: u32::from_be_bytes(t.try_into().unwrap()),
            raw_data: [0x00, 0x01].to_vec(),
            crc: 0
        };
    }
}