use crate::chunk::chunk::Chunk;
use std::convert::TryInto;
use std::fmt;
use std::fmt::Debug;

pub struct Gama {
    pub image_gamma: u32,
}

impl Gama {
    pub fn data(chunk: Chunk) -> Option<Gama> {
        let t = chunk.ascii_type();

        match &*t {
            "gAMA" => {
                Some(Gama {
                    image_gamma: u32::from_be_bytes(chunk.raw_data[0..=3].try_into().unwrap())
                })
            },
            _ => { None }
        }
    }
}

impl fmt::Debug for Gama {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "gamma: {}/100_000", self.image_gamma)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_data() {
        let t = *b"gAMA";

        let chunk = Chunk {
            len: 0,
            raw_type: u32::from_be_bytes(t.try_into().unwrap()),
            raw_data: [0x00, 0x00, 0x00, 0x01].to_vec(),
            crc: 0
        };
        let gama = Gama::data(chunk).unwrap();
        assert_eq!(gama.image_gamma, 1);
    }
}