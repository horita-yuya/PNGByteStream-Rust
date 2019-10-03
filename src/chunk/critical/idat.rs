use crate::chunk::chunk::Chunk;

pub struct Idat {
    pub data: Vec<u8>,
}

impl Idat {
    pub fn data(chunk: Chunk) -> Idat {
        Idat {
            data: chunk.raw_data,
        }
    }
}