use crate::chunk::chunk::Chunk;
use crate::utility::rules::invalid_chunk_type;
use std::fmt;

pub struct Plte {
    pub palettes: Vec<PltePalatte>,
}

#[derive(Copy, Clone)]
pub struct PltePalatte {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Plte {
    pub fn data(chunk: &Chunk) -> Option<Plte> {
        let t = chunk.ascii_type();

        let mut plts: Vec<PltePalatte> = Vec::new();
        for i in (0 ..= chunk.raw_data.len() - 3).step_by(3) {
            let plt = PltePalatte {
                r: chunk.raw_data[i],
                g: chunk.raw_data[i + 1],
                b: chunk.raw_data[i + 2],
            };
            plts.push(plt);
        }

        match &*t {
            "PLTE" => {
                Some(Plte {
                    palettes: plts,
                })
            },
            _ => { None }
        }
    }
}

impl fmt::Debug for Plte {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "total palette count: {}", self.palettes.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_data() {
        let data = [
            0x01, 0x02, 0x03, 0x04,
            0x05, 0x06,
        ];

        let chunk = Chunk {
            len: 0,
            raw_type: 0x504c5445,
            raw_data: data.to_vec(),
            crc: 0
        };

        let plte = Plte::data(&chunk).unwrap();

        assert_eq!(plte.palettes.len(), 2);
        assert_eq!(plte.palettes[0].r, 1);
        assert_eq!(plte.palettes[0].g, 2);
        assert_eq!(plte.palettes[0].b, 3);
        assert_eq!(plte.palettes[1].r, 4);
        assert_eq!(plte.palettes[1].g, 5);
        assert_eq!(plte.palettes[1].b, 6);
    }
}