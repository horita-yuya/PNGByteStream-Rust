use crate::chunk::chunk::Chunk;
use std::convert::TryInto;
use std::fmt;

pub struct Phys {
    pub pixel_per_unit_x: u32,
    pub pixel_per_unit_y: u32,
    pub unit_specifier: u8,
}

impl Phys {
    pub fn data(chunk: Chunk) -> Option<Phys> {
        let t = chunk.ascii_type();

        match &*t {
            "pHYs" => {
                let pixel_per_unit_x = u32::from_be_bytes(chunk.raw_data[0..=3].try_into().unwrap());
                let pixel_per_unit_y = u32::from_be_bytes(chunk.raw_data[4..=7].try_into().unwrap());

                Some(Phys {
                    pixel_per_unit_x,
                    pixel_per_unit_y,
                    unit_specifier: chunk.raw_data[8],
                })
            },
            _ => { None }
        }
    }
}

impl fmt::Debug for Phys {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "pixel per unit x/y: {}/{}, unit specifier: {}", self.pixel_per_unit_x, self.pixel_per_unit_y, self.unit_specifier)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn id_data() {
        let t = *b"pHYs";

        let data = [
            0x00, 0x00, 0x00, 0x01,
            0x00, 0x00, 0x00, 0x02,
            0x03,
        ];
        let chunk = Chunk {
            len: 0,
            raw_type: u32::from_be_bytes(t.try_into().unwrap()),
            raw_data: data.to_vec(),
            crc: 0
        };
        let phys = Phys::data(chunk).unwrap();
        assert_eq!(phys.pixel_per_unit_x, 1);
        assert_eq!(phys.pixel_per_unit_y, 2);
        assert_eq!(phys.unit_specifier, 3);
    }

    #[test]
    fn id_large_data() {
        let t = *b"pHYs";

        let data = [
            0x10, 0x00, 0x00, 0x00,
            0x20, 0x00, 0x00, 0x00,
            0x03,
        ];
        let chunk = Chunk {
            len: 0,
            raw_type: u32::from_be_bytes(t.try_into().unwrap()),
            raw_data: data.to_vec(),
            crc: 0
        };
        let phys = Phys::data(chunk).unwrap();
        assert_eq!(phys.pixel_per_unit_x, 268435456);
        assert_eq!(phys.pixel_per_unit_y, 536870912);
        assert_eq!(phys.unit_specifier, 3);
    }
}