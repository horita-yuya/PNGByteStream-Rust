use std::io;
use std::env;
use chunk::chunk::gen_chunk;

mod chunk;
mod utility;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let url = args.get(1).unwrap();
    let chunks = gen_chunk(url)?;

    println!("{:?}", chunks);

    Ok(())
}