use crate::ihdr::{IHDR, Chunk};
use std::io;
use std::env;
use std::fs::File;
use std::io::Read;
use std::thread::current;

mod ihdr;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let url = "/Users/yuya/Documents/camp/Newton.png";
//    let url = args.get(1).unwrap();
    let chunks = gen_chunk(url)?;

    println!("{:?}", chunks);

    Ok(())
}

fn gen_chunk(url: &str) -> io::Result<Vec<Chunk>> {
    let mut file = File::open(url)?;
    let mut bytes: Vec<u8> = Vec::new();
    file.read_to_end(&mut bytes)?;

    let mut chunks: Vec<Chunk> = Vec::new();
    read_chunk_bytes(&mut chunks, bytes, 8);

    Ok(chunks)
}

fn read_chunk_bytes(chunks: &mut Vec<Chunk>, bytes: Vec<u8>, i: usize) {
    let length = &bytes[i ..= i + 3];
    let raw_type = &bytes[i + 4 ..= i + 7];
    
    let l = (length[0] as usize) << 24
        | (length[1] as usize) << 16
        | (length[2] as usize) << 8
        | (length[3] as usize);

    let t = (raw_type[0] as usize) << 24
        | (raw_type[1] as usize) << 16
        | (raw_type[2] as usize) << 8
        | (raw_type[3] as usize);

    let last_i = i + 11 + l;
    
    let (data, crc) = if l == 0 {
        (Vec::new(), &bytes[i + 8 ..= last_i])

    } else {
        (bytes[i + 8 ..= i + 7 + l].iter().cloned().collect::<Vec<u8>>(), &bytes[i + 8 + l ..= last_i])
    };

    let c = (crc[0] as usize) << 24
        | (crc[1] as usize) << 16
        | (crc[2] as usize) << 8
        | (crc[3] as usize);

    let chunk = Chunk {
        len: l as u32,
        raw_type: t as u32,
        raw_data: data,
        crc: c as u32
    };

    chunks.push(chunk);

    if l != 0 {
        read_chunk_bytes(chunks, bytes, last_i + 1);
    }
}

fn fac(n: usize) -> usize {
    match n {
        0 => { 1 },
        1 => { 1 },
        _ => n * fac(n - 1),
    }
}